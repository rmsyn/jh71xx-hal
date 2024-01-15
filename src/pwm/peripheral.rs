use pac::PWM;

/// Max period length configurable by the HAL.
pub const MAX_PERIOD: u32 = u16::MAX as u32;

/// High-level functions to access low-level PWM PTC registers.
pub trait PwmPeripheral {
    /// Gets the PWM period value.
    ///
    /// This is the number of PWM clock cycles (APB by default).
    fn period(&self) -> u32;
    /// Sets the PWM period value.
    ///
    /// This is the number of PWM clock cycles (APB by default).
    fn set_period(&mut self, val: u32);

    /// Gets the PWM duty-cycle value.
    fn duty(&self) -> u32;
    /// Sets the PWM duty-cycle value.
    ///
    /// The maximum value is the PWM period value.
    ///
    /// If `val` exceeds the period value, duty-cycle will be set to the period.
    fn set_duty(&mut self, val: u32);

    /// Gets whether the PWM is enabled.
    fn enabled(&self) -> bool;
    /// Sets whether to enable the PWM.
    fn enable(&mut self, val: bool);
}

macro_rules! impl_pwm_peripheral {
    ($pwm:ident) => {
        impl $crate::pwm::PwmPeripheral for $pwm {
            fn period(&self) -> u32 {
                self.lrc().read().lrc().bits()
            }
            fn set_period(&mut self, val: u32) {
                self.lrc()
                    .modify(|_, w| w.lrc().variant(core::cmp::min(val, MAX_PERIOD)));
            }

            fn duty(&self) -> u32 {
                self.hrc().read().hrc().bits()
            }
            fn set_duty(&mut self, val: u32) {
                let max = self.period();
                self.hrc()
                    .modify(|_, w| w.hrc().variant(core::cmp::min(val, max)));
            }

            fn enabled(&self) -> bool {
                let r = self.ctrl().read();
                r.en().bit_is_set() && r.oe().bit_is_set()
            }
            fn enable(&mut self, val: bool) {
                self.ctrl().modify(|_, w| match val {
                    false => w.en().clear_bit().oe().clear_bit(),
                    true => w.en().set_bit().oe().set_bit(),
                })
            }
        }
    };
}

// FIXME: JH7110 TRM says the PWM is eight-channel, but there is only one entry in the DTS file...
impl_pwm_peripheral!(PWM);
