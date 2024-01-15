use core::cmp;

use embedded_hal::delay::DelayNs;
use embedded_hal::i2c::{self, I2c as I2cHal, Operation, SevenBitAddress, TenBitAddress};

use crate::{bitflag_is_set, delay::u74_mdelay};

mod constants;
mod error;
mod message;
mod mode;
mod peripheral;
mod registers;
mod timings;

pub use constants::*;
pub use error::*;
pub use message::*;
pub use mode::*;
pub use peripheral::*;
pub use registers::*;
pub use timings::*;

bitflags! {
    /// Software status flags.
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct Status: u32 {
        const NONE = 0b0000;
        const ACTIVE = 0b0001;
        const WRITE_IN_PROGRESS = 0b0010;
        const READ_IN_PROGRESS = 0b0100;
        const MASK = 0b0111;
    }
}

bitflag_is_set!(Status);

/// I2C host
pub struct I2c<I2C: I2cPeripheral> {
    i2c: I2C,
    status: Status,
    rx_fifo_depth: u32,
    tx_fifo_depth: u32,
    tx_buf_len: usize,
    rx_buf_len: usize,
    tx_outstanding: u32,
    rx_outstanding: u32,
    tx_flag: I2cMsgFlag,
    rx_flag: I2cMsgFlag,
    master_cfg: I2cCon,
    functionality: I2cFunc,
    ss_hcnt: u32,
    ss_lcnt: u32,
    fs_hcnt: u32,
    fs_lcnt: u32,
    hs_hcnt: u32,
    hs_lcnt: u32,
    sda_hold_time: u32,
    timings: I2cTimings,
    mode: I2cOpMode,
    msg_err: i32,
}

impl<I2C: I2cPeripheral> I2c<I2C> {
    /// Creates a new [I2c].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use jh71xx_hal::{pac, i2c};
    /// let dp = pac::Peripherals::take().unwrap();
    /// let _i2c0 = i2c::I2c::new(dp.I2C0);
    /// ```
    pub fn new(i2c: I2C) -> Self {
        Self {
            i2c,
            status: Status::default(),
            rx_fifo_depth: 0,
            tx_fifo_depth: 0,
            tx_buf_len: 0,
            rx_buf_len: 0,
            tx_outstanding: 0,
            rx_outstanding: 0,
            tx_flag: I2cMsgFlag::default(),
            rx_flag: I2cMsgFlag::default(),
            functionality: I2cFunc::default(),
            master_cfg: I2cCon::default(),
            ss_hcnt: 0,
            ss_lcnt: 0,
            fs_hcnt: 0,
            fs_lcnt: 0,
            hs_hcnt: 0,
            hs_lcnt: 0,
            sda_hold_time: 0,
            timings: I2cTimings::default(),
            mode: I2cOpMode::default(),
            msg_err: 0,
        }
    }

    /// Gets the [Status].
    pub const fn status(&self) -> Status {
        self.status
    }

    /// Gets the TX FIFO depth.
    pub const fn tx_fifo_depth(&self) -> u32 {
        self.tx_fifo_depth
    }

    /// Gets the RX FIFO depth.
    pub const fn rx_fifo_depth(&self) -> u32 {
        self.rx_fifo_depth
    }

    /// Configures Tx/Rx FIFO thresholds, and sets the device to `master` mode.
    pub fn configure_fifo_master(&mut self) {
        let depth = self.tx_fifo_depth / 2;

        // Configure Tx/Rx FIFO threshold levels
        self.i2c.set_tx_tl(depth);
        self.i2c.set_rx_tl(0);

        // Configure the I2C master
        self.i2c.set_con(self.master_cfg);
    }

    /// Configure the I2C peripheral for `master` operation mode.
    pub fn configure_master(&mut self) {
        self.functionality = I2cFunc::ADDRESS_10BIT | I2cFunc::default();
        self.master_cfg = I2cCon::MASTER | I2cCon::SLAVE_DISABLE | I2cCon::RESTART_EN;

        self.mode = I2cOpMode::Master;

        let bus_freq_hz = self.timings.bus_freq_hz();

        self.master_cfg |= match bus_freq_hz {
            I2cSpeedMode::Standard => I2cCon::SPEED_STD,
            I2cSpeedMode::High => I2cCon::SPEED_HIGH,
            _ => I2cCon::SPEED_FAST,
        };
    }

    fn read_poll_timeout(
        &mut self,
        poll_fn: impl Fn(&mut dyn I2cPeripheral) -> bool,
        sleep_us: u32,
        timeout: u32,
    ) -> Result<()> {
        let mut success = false;
        let mut time = 0;
        let mut delay = u74_mdelay();

        while !success && time <= timeout {
            success = poll_fn(&mut self.i2c);
            if success {
                break;
            }
            delay.delay_us(sleep_us);
            time = time.saturating_add(sleep_us);
        }

        if success {
            Ok(())
        } else {
            Err(Error::Other)
        }
    }

    fn __enable(&mut self) {
        self.i2c.set_enable(I2cEnable::ENABLE);
    }

    fn __disable(&mut self) {
        let raw_intr_stat = self.i2c.get_raw_interrupt_stat();
        let enable = self.i2c.get_enable();

        let abort_needed = raw_intr_stat.is_set(I2cRawInterruptStatus::MST_ON_HOLD)
            && !enable.is_set(I2cEnable::ABORT);

        if abort_needed {
            self.i2c.set_enable(I2cEnable::ABORT);
            if let Err(_err) =
                self.read_poll_timeout(|i2c| i2c.get_enable().is_set(I2cEnable::ABORT), 10, 100)
            {
                // FIXME: implement uart logging
                // defmt_log!("timeout while trying to abort current transfer");
                return;
            }
        }

        let timeout = 100;
        let mut delay = u74_mdelay();

        for _ in (0..timeout).rev() {
            self.__disable_nowait();
            if self
                .i2c
                .get_enable_status()
                .is_set(I2cEnableStatus::ACTIVITY)
            {
                return;
            }

            // Wait 10 times the signaling period of the highest I2C
            // transfer supported by the driver (for 400KHz this is
            // 25us) as described in the DesignWare I2C databook.
            delay.delay_us(25);
        }
        // FIXME: implement uart logging
        // defmt_log!("timeout in disabling adapter");
    }

    fn __disable_nowait(&mut self) {
        self.i2c.set_enable(I2cEnable::NONE);
    }

    /// Initializes the Designware I2C master hardware.
    ///
    /// This function configures and enables the I2C master.
    ///
    /// This function is called during I2C init funciton, and in case of timeout at run-time.
    pub fn init_master(&mut self) {
        // Disable the adapter
        self.__disable();

        // Write standard speed timing parameters
        self.i2c.set_ss_scl_hcnt(self.ss_hcnt);
        self.i2c.set_ss_scl_lcnt(self.ss_lcnt);

        // Write fast mode/fast mode plus timing parameters
        self.i2c.set_fs_scl_hcnt(self.fs_hcnt);
        self.i2c.set_fs_scl_lcnt(self.fs_lcnt);

        // Write high speed timing parameters if supported
        if self.hs_hcnt != 0 && self.hs_lcnt != 0 {
            self.i2c.set_hs_scl_hcnt(self.hs_hcnt);
            self.i2c.set_hs_scl_lcnt(self.hs_lcnt);
        }

        // Write SDA hold time if supported
        if self.sda_hold_time != 0 {
            let hold_time = self.sda_hold_time;
            self.i2c.set_sda_hold(hold_time);
        }

        self.configure_fifo_master();
    }

    /// Prepares the I2C peripheral for transfer(s).
    pub fn xfer_init(&mut self, tar: I2cTar) {
        // Disable the adapter.
        self.__disable();

        let con = if tar.is_set(I2cTar::MODE_10BIT) {
            I2cCon::MASTER_10BIT
        } else {
            I2cCon::NONE
        };

        self.i2c.set_con(con);
        self.i2c.set_tar(tar);

        // Enforce disabled interrupts (due to HW issues)
        // TODO: this is a problem with some (all?) platforms Linux supports.
        // Check if the problem exists for JH71xx hardware.
        self.i2c.set_interrupt_mask(I2cInterruptMask::NONE);

        // Enable the adapter
        self.__enable();

        // Dummy read to avoid the register getting stuck
        // TODO: Linux driver does this for Bay Trail.
        // Check if this is necessary for JH71xx hardware.
        let _en_stat = self.i2c.get_enable_status();

        // Clear and enable interrupts
        let _ci = self.i2c.get_clear_interrupt();
        self.i2c.set_interrupt_mask(I2cInterruptMask::master());
    }

    /// Initiates (and continues) low level master read/write transaction.
    pub fn write_msg(&mut self, buf: &[u8], last_msg: bool) -> Result<()> {
        let mut need_restart = !self.status.is_set(Status::WRITE_IN_PROGRESS)
            && self.master_cfg.is_set(I2cCon::RESTART_EN);

        let tx_limit = self.tx_fifo_depth.saturating_sub(self.i2c.get_txflr()) as usize;
        let len = cmp::min(buf.len(), tx_limit);

        for (i, data_byte) in buf[..len].iter().enumerate() {
            let mut cmd = I2cDataCmd::NONE;

            // i2c-core always sets the buffer length of
            // I2C_FUNC_SMBUS_BLOCK_DATA to 1. The length will
            // be adjusted when receiving the first byte.
            // Thus we can't stop the transaction here.
            if last_msg && i == len.saturating_sub(1) && !self.tx_flag.is_set(I2cMsgFlag::RECV_LEN)
            {
                cmd |= I2cDataCmd::STOP;
            }

            if need_restart {
                cmd |= I2cDataCmd::RESTART;
                need_restart = false;
            }

            // use the checked index to ensure we don't panic
            self.i2c.set_data_cmd(cmd | I2cDataCmd::from(data_byte));
        }

        if len > tx_limit || self.tx_flag.is_set(I2cMsgFlag::RECV_LEN) {
            self.status |= Status::WRITE_IN_PROGRESS;
            self.tx_outstanding = self.tx_outstanding.saturating_add(1);
            self.tx_buf_len = len.saturating_sub(tx_limit);
        } else {
            self.status &= !Status::WRITE_IN_PROGRESS;
        }

        let intr_mask = if self.msg_err != 0 {
            I2cInterruptMask::NONE
        } else if last_msg {
            I2cInterruptMask::master() & !I2cInterruptMask::TX_EMPTY
        } else {
            I2cInterruptMask::master()
        };

        self.i2c.set_interrupt_mask(intr_mask);

        Ok(())
    }

    /// Reads a message from the RX FIFO buffer.
    ///
    /// **NOTE**: HAL users should check [I2c::status()] and [I2c::rx_fifo_depth()]
    /// for any additional bytes that remain on the bus.
    ///
    /// Users should set [`Operation::Read`] buffers to have a length at least
    /// the RX FIFO buffer depth to avoid making multiple read calls.
    pub fn read_msg(&mut self, buf: &mut [u8]) -> Result<()> {
        // Avoid RX buffer overrun
        if self.rx_outstanding >= self.rx_fifo_depth {
            return Err(Error::Overrun);
        }

        let cmd = if self.master_cfg.is_set(I2cCon::RESTART_EN) {
            I2cDataCmd::READ | I2cDataCmd::RESTART
        } else {
            I2cDataCmd::READ
        };

        self.i2c.set_data_cmd(cmd);
        // Actual read happens in the interrupt handler I2c::isr() that calls
        // I2c::read(). This is because the peripheral fills an RX FIFO,
        // and interrupts when the FIFO is full.
        //
        // Wait until the interrupt register indicates a full FIFO buffer.
        self.read_poll_timeout(
            |i2c| {
                let (stat, _) = i2c.read_clear_interrupt();
                stat.is_set(I2cInterruptStatus::RX_FULL)
            },
            10,
            100,
        )?;

        let rx_valid = self.i2c.get_rxflr() as usize;

        let len = cmp::min(buf.len(), rx_valid);

        for dst in buf[..len].iter_mut() {
            let mut tmp = self.i2c.get_data_cmd().data();
            if self.rx_flag.is_set(I2cMsgFlag::RECV_LEN) {
                // if IC_EMPTYFIFO_HOLD_MASTER_EN is set, which cannot be
                // detected from the registers, the controller can be
                // disabled if the STOP bit is set. But it is only set
                // after receiving block data response length in
                // I2C_FUNC_SMBUS_BLOCK_DATA case. That needs to read
                // another byte with STOP bit set when the block data
                // response length is invalid to complete the transaction.
                if tmp == 0 || tmp > I2C_SMBUS_BLOCK_MAX {
                    tmp = 1;
                }

                // NOTE: Linux driver buffers read messages in a local buffer,
                // however this buffering should be handled by HAL users.
            }
            *dst = tmp;
        }

        if rx_valid > len {
            // NOTE: HAL users should check I2c::status() and I2c::rx_buf_len()
            // after I2c::read() calls for additional bytes that remain on the bus.
            self.status &= Status::READ_IN_PROGRESS;
            self.rx_buf_len = rx_valid.saturating_sub(len);
        } else {
            self.status &= !Status::READ_IN_PROGRESS;
            self.rx_buf_len = 0;
        }

        Ok(())
    }
}

impl<I2C: I2cPeripheral> i2c::ErrorType for I2c<I2C> {
    type Error = Error;
}

impl<I2C: I2cPeripheral> I2cHal<SevenBitAddress> for I2c<I2C> {
    fn transaction(&mut self, address: u8, operations: &mut [Operation<'_>]) -> Result<()> {
        let tar = I2cTar::from(address as u32);
        self.xfer_init(tar);

        let mut writes = operations
            .iter()
            .filter(|o| matches!(o, Operation::Write(_)))
            .count();

        for op in operations.iter_mut() {
            match op {
                Operation::Read(xfer) => self.read_msg(xfer)?,
                Operation::Write(xfer) => {
                    writes = writes.saturating_sub(1);
                    self.write_msg(xfer, writes == 0)?;
                }
            }
        }

        Ok(())
    }
}

impl<I2C: I2cPeripheral> I2cHal<TenBitAddress> for I2c<I2C> {
    fn transaction(&mut self, address: u16, operations: &mut [Operation<'_>]) -> Result<()> {
        let tar = I2cTar::from(address as u32) | I2cTar::MODE_10BIT;
        self.xfer_init(tar);

        let mut writes = operations
            .iter()
            .filter(|o| matches!(o, Operation::Write(_)))
            .count();

        for op in operations.iter_mut() {
            match op {
                Operation::Read(xfer) => self.read_msg(xfer)?,
                Operation::Write(xfer) => {
                    writes = writes.saturating_sub(1);
                    self.write_msg(xfer, writes == 0)?;
                }
            }
        }

        Ok(())
    }
}
