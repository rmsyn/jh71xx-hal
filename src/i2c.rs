use core::any::Any;

use embedded_hal::delay::DelayUs;
use embedded_hal::i2c::{self, AddressMode, I2c as I2cHal, Operation, SevenBitAddress, TenBitAddress};

use crate::{bitflag_is_set, delay::u74_mdelay};

mod constants;
mod error;
mod message;
mod mode;
mod timings;

pub use constants::*;
pub use error::*;
pub use message::*;
pub use mode::*;
pub use timings::*;

bitflags! {
    /// Represents I2C `CON` register bitfield.
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct I2cCon: u32 {
        const NONE = 0b0000_0000_0000_0000;
        const MASTER = 0b0000_0000_0000_0001;
        const SPEED_STD = 0b0000_0000_0000_0010;
        const SPEED_FAST = 0b0000_0000_0000_0100;
        const SPEED_HIGH = 0b0000_0000_0000_0110;
        const SLAVE_10BIT = 0b0000_0000_0000_1000;
        const MASTER_10BIT = 0b0000_0000_0001_0000;
        const RESTART_EN = 0b0000_0000_0010_0000;
        const SLAVE_DISABLE = 0b0000_0000_0100_0000;
        const STOP_DET_IFADDRESSED = 0b0000_0000_1000_0000;
        const TX_EMPTY_CTRL = 0b0000_0001_0000_0000;
        const RX_FIFO_FULL_HLD_CTRL = 0b0000_0010_0000_0000;
        const BUS_CLEAR_CTRL = 0b0000_1000_0000_0000;
    }
}

bitflag_is_set!(I2cCon);

bitflags! {
    /// Represents I2C `RAW_INTR_STAT` register bitfield.
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct I2cRawInterruptStatus: u32 {
        const NONE = 0b0000_0000_0000_0000;
        const RX_UNDER = 0b0000_0000_0000_0001;
        const RX_OVER = 0b0000_0000_0000_0010;
        const RX_FULL = 0b0000_0000_0000_0100;
        const TX_OVER = 0b0000_0000_0000_1000;
        const TX_EMPTY = 0b0000_0000_0001_0000;
        const RD_REQ = 0b0000_0000_0010_0000;
        const TX_ABRT = 0b0000_0000_0100_0000;
        const RX_DONE = 0b0000_0000_1000_0000;
        const ACTIVITY = 0b0000_0001_0000_0000;
        const STOP_DET = 0b0000_0010_0000_0000;
        const START_DET = 0b0000_0100_0000_0000;
        const GEN_CALL = 0b0000_1000_0000_0000;
        const RESTART_DET = 0b0001_0000_0000_0000;
        const MST_ON_HOLD = 0b0010_0000_0000_0000;
    }
}

bitflag_is_set!(I2cRawInterruptStatus);

bitflags! {
    /// Represents I2C `ENABLE` register bitfield.
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct I2cEnable: u32 {
        const NONE = 0b0000_0000_0000_0000;
        const ENABLE = 0b0000_0000_0000_0001;
        const ABORT = 0b0000_0000_0000_0010;
    }
}

bitflag_is_set!(I2cEnable);

bitflags! {
    /// Represents I2C `ENABLE` register bitfield.
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct I2cEnableStatus: u32 {
        const NONE = 0b0000_0000_0000_0000;
        const ACTIVITY = 0b0000_0000_0000_0001;
        const TFE = 0b0000_0000_0000_0100;
        const RFNE = 0b0000_0000_0000_1000;
        const MASTER_ACTIVITY = 0b0000_0000_0010_0000;
        const SLAVE_ACTIVITY = 0b0000_0000_0100_0000;
    }
}

bitflag_is_set!(I2cEnableStatus);

bitflags! {
    /// Represents I2C functionality bitfield.
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct I2cFunc: u32 {
        const NONE = 0x0000_0000;
        const I2C = 0x0000_0001;
        const ADDRESS_10BIT = 0x0000_0002;
        const PROTOCOL_MANGLING = 0x0000_0004;
        const SMBUS_PEC = 0x0000_0008;
        const NOSTART = 0x0000_0010;
        const SLAVE = 0x0000_0020;
        const SMBUS_BLOCK_PROC_CALL = 0x0000_8000;
        const SMBUS_QUICK = 0x0001_0000;
        const SMBUS_READ_BYTE = 0x0002_0000;
        const SMBUS_WRITE_BYTE = 0x0004_0000;
        const SMBUS_READ_BYTE_DATA = 0x0008_0000;
        const SMBUS_WRITE_BYTE_DATA = 0x0010_0000;
        const SMBUS_READ_WORD_DATA = 0x0020_0000;
        const SMBUS_WRITE_WORD_DATA = 0x0040_0000;
        const SMBUS_PROC_CALL = 0x0080_0000;
        const SMBUS_READ_BLOCK_DATA = 0x0100_0000;
        const SMBUS_WRITE_BLOCK_DATA = 0x0200_0000;
        const SMBUS_READ_I2C_BLOCK = 0x0400_0000;
        const SMBUS_WRITE_I2C_BLOCK = 0x0800_0000;
        const SMBUS_HOST_NOTIFY = 0x1000_0000;
        const SMBUS_BYTE = 0x0002_0000 | 0x0004_0000;
        const SMBUS_BYTE_DATA = 0x0008_0000 | 0x0010_0000;
        const SMBUS_WORD_DATA = 0x0020_0000 | 0x0040_0000;
        const SMBUS_BLOCK_DATA = 0x0100_0000 | 0x0200_0000;
        const SMBUS_I2C_BLOCK = 0x0400_0000 | 0x0800_0000;
    }
}

impl Default for I2cFunc {
    fn default() -> Self {
        I2cFunc::I2C
            | I2cFunc::SMBUS_BYTE
            | I2cFunc::SMBUS_BYTE_DATA
            | I2cFunc::SMBUS_WORD_DATA
            | I2cFunc::SMBUS_BLOCK_DATA
            | I2cFunc::SMBUS_I2C_BLOCK
    }
}

/// Generic access for Synopsis Designware I2C peripherals.
pub trait I2cPeripheral {
    fn con(&self) -> I2cCon;
    fn set_con(&mut self, flags: I2cCon);

    fn tx_tl(&self) -> u32;
    fn set_tx_tl(&mut self, val: u32);

    fn rx_tl(&self) -> u32;
    fn set_rx_tl(&mut self, val: u32);

    fn ss_scl_hcnt(&self) -> u32;
    fn set_ss_scl_hcnt(&mut self, val: u32);

    fn ss_scl_lcnt(&self) -> u32;
    fn set_ss_scl_lcnt(&mut self, val: u32);

    fn fs_scl_hcnt(&self) -> u32;
    fn set_fs_scl_hcnt(&mut self, val: u32);

    fn fs_scl_lcnt(&self) -> u32;
    fn set_fs_scl_lcnt(&mut self, val: u32);

    fn hs_scl_hcnt(&self) -> u32;
    fn set_hs_scl_hcnt(&mut self, val: u32);

    fn hs_scl_lcnt(&self) -> u32;
    fn set_hs_scl_lcnt(&mut self, val: u32);

    fn sda_hold(&self) -> u32;
    fn set_sda_hold(&mut self, val: u32);

    fn raw_intr_stat(&self) -> I2cRawInterruptStatus;
    fn set_raw_intr_stat(&mut self, val: I2cRawInterruptStatus);

    fn enable(&self) -> I2cEnable;
    fn set_enable(&mut self, val: I2cEnable);

    fn enable_status(&self) -> I2cEnableStatus;
    fn set_enable_status(&mut self, val: I2cEnableStatus);
}

/// I2C host
pub struct I2c<I2C: I2cPeripheral> {
    i2c: I2C,
    rx_fifo_depth: u32,
    tx_fifo_depth: u32,
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
}

impl<I2C: I2cPeripheral> I2c<I2C> {
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
        let raw_intr_stat = self.i2c.raw_intr_stat();
        let enable = self.i2c.enable();

        let abort_needed = raw_intr_stat.is_set(I2cRawInterruptStatus::MST_ON_HOLD)
            && !enable.is_set(I2cEnable::ABORT);

        if abort_needed {
            self.i2c.set_enable(I2cEnable::ABORT);
            if let Err(_err) =
                self.read_poll_timeout(|i2c| i2c.enable().is_set(I2cEnable::ABORT), 10, 100)
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
            if self.i2c.enable_status().is_set(I2cEnableStatus::ACTIVITY) {
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

    pub fn xfer_init<A: AddressMode>(&mut self, a: A) {
        // Disable the adapter.
        self.__disable();

        let (ic_con, ic_tar) = if a.is_ten_bit() {
            (I2cCon::MASTER_10BIT, I2cTar::MASTER_10BIT)
        } else {
            (I2cCon::new(), I2cTar::new())
        };

        // FIXME: finish
    }
}

impl<I2C: I2cPeripheral> i2c::ErrorType for I2c<I2C> {
    type Error = Error;
}

/// Gets whether the an [`AddressMode`](embedded_hal::i2c::AddressMode) is a
/// [`SevenBitAddress`](embedded_hal::i2c::SevenBitAddress).
pub fn is_seven_bit(a: &dyn Any) -> bool {
    a.is::<SevenBitAddress>()
}

/// Gets whether the an [`AddressMode`](embedded_hal::i2c::AddressMode) is a
/// [`TenBitAddress`](embedded_hal::i2c::TenBitAddress).
pub fn is_ten_bit(a: &dyn Any) -> bool {
    a.is::<TenBitAddress>()
}

impl<I2C: I2cPeripheral> I2cHal<SevenBitAddress> for I2c<I2C> {
    fn transaction(&mut self, _address: u8, operations: &mut [Operation<'_>]) -> Result<()> {
        for op in operations.iter_mut() {
            match op {
                Operation::Read(_xfer) => {
                    unimplemented!("FIXME: implement");
                }
                Operation::Write(_xfer) => {
                    unimplemented!("FIXME: implement");
                }
            }
        }

        Ok(())
    }
}
