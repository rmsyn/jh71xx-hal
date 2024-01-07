pub use crate::pac::{I2C0, I2C1, I2C2, I2C3, I2C4, I2C5, I2C6};

use super::registers::*;

/// Generic access for Synopsis Designware I2C peripherals.
// FIXME: add `modify_*` methods to only modify set bitfields.
pub trait I2cPeripheral {
    fn get_con(&self) -> I2cCon;
    fn set_con(&mut self, val: I2cCon);

    fn get_tar(&self) -> I2cTar;
    fn set_tar(&mut self, val: I2cTar);

    fn get_sar(&self) -> I2cSar;
    fn set_sar(&mut self, val: I2cSar);

    fn get_tx_tl(&self) -> u32;
    fn set_tx_tl(&mut self, val: u32);

    fn get_rx_tl(&self) -> u32;
    fn set_rx_tl(&mut self, val: u32);

    fn get_ss_scl_hcnt(&self) -> u32;
    fn set_ss_scl_hcnt(&mut self, val: u32);

    fn get_ss_scl_lcnt(&self) -> u32;
    fn set_ss_scl_lcnt(&mut self, val: u32);

    fn get_fs_scl_hcnt(&self) -> u32;
    fn set_fs_scl_hcnt(&mut self, val: u32);

    fn get_fs_scl_lcnt(&self) -> u32;
    fn set_fs_scl_lcnt(&mut self, val: u32);

    fn get_hs_scl_hcnt(&self) -> u32;
    fn set_hs_scl_hcnt(&mut self, val: u32);

    fn get_hs_scl_lcnt(&self) -> u32;
    fn set_hs_scl_lcnt(&mut self, val: u32);

    fn get_sda_hold(&self) -> u32;
    fn set_sda_hold(&mut self, val: u32);

    fn get_raw_interrupt_stat(&self) -> I2cRawInterruptStatus;

    fn get_interrupt_stat(&self) -> I2cInterruptStatus;

    fn get_interrupt_mask(&self) -> I2cInterruptMask;
    fn set_interrupt_mask(&mut self, val: I2cInterruptMask);

    fn get_clear_interrupt(&self) -> I2cClearInterrupt;
    fn set_clear_interrupt(&mut self, val: I2cClearInterrupt);

    fn get_clear_rx_under(&self) -> u32;
    fn get_clear_rx_over(&self) -> u32;
    fn get_clear_tx_over(&self) -> u32;
    fn get_clear_rd_req(&self) -> u32;
    fn get_clear_tx_abort(&self) -> u32;
    fn get_clear_rx_done(&self) -> u32;
    fn get_clear_activity(&self) -> u32;
    fn get_clear_stop_det(&self) -> u32;
    fn get_clear_start_det(&self) -> u32;
    fn get_clear_gen_call(&self) -> u32;

    fn get_enable(&self) -> I2cEnable;
    fn set_enable(&mut self, val: I2cEnable);

    fn get_enable_status(&self) -> I2cEnableStatus;
    fn set_enable_status(&mut self, val: I2cEnableStatus);

    fn get_txflr(&self) -> u32;
    fn set_txflr(&mut self, val: u32);

    fn get_rxflr(&self) -> u32;
    fn set_rxflr(&mut self, val: u32);

    fn get_data_cmd(&self) -> I2cDataCmd;
    fn set_data_cmd(&mut self, val: I2cDataCmd);

    fn get_tx_abort_source(&self) -> I2cTxAbortSource;

    fn read_clear_interrupt(&self) -> (I2cInterruptStatus, I2cTxAbortSource) {
        // The `INTR_STAT` register just indicates "enabled" interrupts.
        // The unmasked raw version of interrupt status bits is available
        // in the `RAW_INTR_STAT` register.
        //
        // That is,
        //   stat = readl(`INTR_STAT`);
        // equals to,
        //   stat = readl(`RAW_INTR_STAT`) & readl(`INTR_MASK`);
        //
        // The raw version might be useful for debugging purposes.
        let stat = self.get_interrupt_stat();

        // Do not use the IC_CLR_INTR register to clear interrupts, or
        // you'll miss some interrupts, triggered during the period from
        // readl(`INTR_STAT`) to readl(`CLR_INTR`).
        //
        // Instead, use the separately-prepared `I2cPeripheral::clear_*()` methods.
        if stat.is_set(I2cInterruptStatus::RX_UNDER) {
            self.get_clear_rx_under();
        }
        if stat.is_set(I2cInterruptStatus::RX_OVER) {
            self.get_clear_rx_over();
        }
        if stat.is_set(I2cInterruptStatus::TX_OVER) {
            self.get_clear_tx_over();
        }
        if stat.is_set(I2cInterruptStatus::RD_REQ) {
            self.get_clear_rd_req();
        }
        let tx_abort_source = if stat.is_set(I2cInterruptStatus::TX_ABRT) {
            // The `TX_ABRT_SOURCE` register is cleared whenever the `CLR_TX_ABRT` register is read.
            // Preserve it beforehand.
            let src = self.get_tx_abort_source();
            self.get_clear_tx_abort();
            src
        } else {
            I2cTxAbortSource::NONE
        };
        if stat.is_set(I2cInterruptStatus::RX_DONE) {
            self.get_clear_rx_done();
        }
        if stat.is_set(I2cInterruptStatus::ACTIVITY) {
            self.get_clear_activity();
        }
        if stat.is_set(I2cInterruptStatus::STOP_DET) {
            self.get_clear_stop_det();
        }
        if stat.is_set(I2cInterruptStatus::START_DET) {
            self.get_clear_start_det();
        }
        if stat.is_set(I2cInterruptStatus::GEN_CALL) {
            self.get_clear_gen_call();
        }

        (stat, tx_abort_source)
    }
}

macro_rules! impl_i2c_peripheral {
    ($i2c:ident) => {
        impl $crate::i2c::I2cPeripheral for $i2c {
            fn get_con(&self) -> I2cCon {
                I2cCon::from(self.con().read().bits())
            }

            /// SAFETY: setting register bits is safe because of guarantees made by the [I2cCon]
            /// bitflag invariants.
            fn set_con(&mut self, val: I2cCon) {
                self.con().write(|w| unsafe { w.bits(val.bits()) });
            }

            fn get_tar(&self) -> I2cTar {
                I2cTar::from(self.tar().read().bits())
            }

            /// SAFETY: setting register bits is safe because of guarantees made by the [I2cTar]
            /// bitflag invariants.
            fn set_tar(&mut self, val: I2cTar) {
                self.tar().write(|w| unsafe { w.bits(val.bits()) });
            }

            fn get_sar(&self) -> I2cSar {
                I2cSar::from(self.sar().read().bits())
            }

            /// SAFETY: setting register bits is safe because of guarantees made by the [I2cSar]
            /// bitflag invariants.
            fn set_sar(&mut self, val: I2cSar) {
                self.sar().write(|w| unsafe { w.bits(val.bits()) });
            }

            fn get_tx_tl(&self) -> u32 {
                self.tx_tl().read().bits()
            }

            fn set_tx_tl(&mut self, val: u32) {
                self.tx_tl().write(|w| w.tx_tl().variant(val))
            }

            fn get_rx_tl(&self) -> u32 {
                self.rx_tl().read().bits()
            }

            fn set_rx_tl(&mut self, val: u32) {
                self.rx_tl().write(|w| w.rx_tl().variant(val));
            }

            fn get_ss_scl_hcnt(&self) -> u32 {
                self.ss_scl_hcnt().read().bits()
            }

            fn set_ss_scl_hcnt(&mut self, val: u32) {
                self.ss_scl_hcnt().write(|w| w.ss_scl_hcnt().variant(val));
            }

            fn get_ss_scl_lcnt(&self) -> u32 {
                self.ss_scl_lcnt().read().bits()
            }

            fn set_ss_scl_lcnt(&mut self, val: u32) {
                self.ss_scl_lcnt().write(|w| w.ss_scl_lcnt().variant(val));
            }

            fn get_fs_scl_hcnt(&self) -> u32 {
                self.fs_scl_hcnt().read().bits()
            }

            fn set_fs_scl_hcnt(&mut self, val: u32) {
                self.fs_scl_hcnt().write(|w| w.fs_scl_hcnt().variant(val));
            }

            fn get_fs_scl_lcnt(&self) -> u32 {
                self.fs_scl_lcnt().read().bits()
            }

            fn set_fs_scl_lcnt(&mut self, val: u32) {
                self.fs_scl_lcnt().write(|w| w.fs_scl_lcnt().variant(val));
            }

            fn get_hs_scl_hcnt(&self) -> u32 {
                self.hs_scl_hcnt().read().bits()
            }

            fn set_hs_scl_hcnt(&mut self, val: u32) {
                self.hs_scl_hcnt().write(|w| w.hs_scl_hcnt().variant(val));
            }

            fn get_hs_scl_lcnt(&self) -> u32 {
                self.hs_scl_lcnt().read().bits()
            }

            fn set_hs_scl_lcnt(&mut self, val: u32) {
                self.hs_scl_lcnt().write(|w| w.hs_scl_lcnt().variant(val));
            }

            fn get_sda_hold(&self) -> u32 {
                self.hs_scl_lcnt().read().bits()
            }

            fn set_sda_hold(&mut self, val: u32) {
                self.sda_hold().write(|w| w.sda_hold().variant(val));
            }

            fn get_raw_interrupt_stat(&self) -> I2cRawInterruptStatus {
                I2cRawInterruptStatus::from(self.raw_intr_stat().read().bits())
            }

            fn get_interrupt_stat(&self) -> I2cInterruptStatus {
                I2cInterruptStatus::from(self.intr_stat().read().bits())
            }

            fn get_interrupt_mask(&self) -> I2cInterruptMask {
                I2cInterruptMask::from(self.intr_mask().read().bits())
            }

            /// SAFETY: setting register bits is safe because of guarantees made by the
            /// [I2cInterruptMask] bitflag invariants.
            fn set_interrupt_mask(&mut self, val: I2cInterruptMask) {
                self.intr_mask().write(|w| unsafe { w.bits(val.bits()) });
            }

            fn get_clear_interrupt(&self) -> I2cClearInterrupt {
                I2cClearInterrupt::from(self.clr_intr().read().bits())
            }

            /// SAFETY: setting register bits is safe because of guarantees made by the
            /// [I2cClearInterrupt] bitflag invariants.
            fn set_clear_interrupt(&mut self, val: I2cClearInterrupt) {
                self.clr_intr().write(|w| unsafe { w.bits(val.bits()) });
            }

            fn get_clear_rx_under(&self) -> u32 {
                self.clr_rx_under().read().bits()
            }

            fn get_clear_rx_over(&self) -> u32 {
                self.clr_rx_over().read().bits()
            }

            fn get_clear_tx_over(&self) -> u32 {
                self.clr_tx_over().read().bits()
            }

            fn get_clear_rd_req(&self) -> u32 {
                self.clr_rd_req().read().bits()
            }

            fn get_clear_tx_abort(&self) -> u32 {
                self.clr_tx_abrt().read().bits()
            }

            fn get_clear_rx_done(&self) -> u32 {
                self.clr_rx_done().read().bits()
            }

            fn get_clear_activity(&self) -> u32 {
                self.clr_activity().read().bits()
            }

            fn get_clear_stop_det(&self) -> u32 {
                self.clr_stop_det().read().bits()
            }

            fn get_clear_start_det(&self) -> u32 {
                self.clr_start_det().read().bits()
            }

            fn get_clear_gen_call(&self) -> u32 {
                self.clr_gen_call().read().bits()
            }

            fn get_enable(&self) -> I2cEnable {
                I2cEnable::from(self.clr_gen_call().read().bits())
            }

            /// SAFETY: setting register bits is safe because of guarantees made by the
            /// [I2cEnable] bitflag invariants.
            fn set_enable(&mut self, val: I2cEnable) {
                self.enable().write(|w| unsafe { w.bits(val.bits()) });
            }

            fn get_enable_status(&self) -> I2cEnableStatus {
                I2cEnableStatus::from(self.enable_status().read().bits())
            }

            /// SAFETY: setting register bits is safe because of guarantees made by the
            /// [I2cEnableStatus] bitflag invariants.
            fn set_enable_status(&mut self, val: I2cEnableStatus) {
                self.enable().write(|w| unsafe { w.bits(val.bits()) });
            }

            fn get_txflr(&self) -> u32 {
                self.txflr().read().bits()
            }

            fn set_txflr(&mut self, val: u32) {
                self.txflr().write(|w| w.txflr().variant(val));
            }

            fn get_rxflr(&self) -> u32 {
                self.rxflr().read().bits()
            }

            fn set_rxflr(&mut self, val: u32) {
                self.rxflr().write(|w| w.rxflr().variant(val));
            }

            fn get_data_cmd(&self) -> I2cDataCmd {
                I2cDataCmd::from(self.data_cmd().read().bits())
            }

            /// SAFETY: setting register bits is safe because of guarantees made by the
            /// [I2cDataCmd] bitflag invariants.
            fn set_data_cmd(&mut self, val: I2cDataCmd) {
                self.data_cmd().write(|w| unsafe { w.bits(val.bits()) });
            }

            fn get_tx_abort_source(&self) -> I2cTxAbortSource {
                I2cTxAbortSource::from(self.tx_abrt_source().read().bits())
            }
        }
    };
}

impl_i2c_peripheral!(I2C0);
impl_i2c_peripheral!(I2C1);
impl_i2c_peripheral!(I2C2);
impl_i2c_peripheral!(I2C3);
impl_i2c_peripheral!(I2C4);
impl_i2c_peripheral!(I2C5);
impl_i2c_peripheral!(I2C6);
