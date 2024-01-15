use embedded_hal::pwm::{ErrorType, SetDutyCycle};

mod error;
mod peripheral;

pub use error::*;
pub use peripheral::*;

/// Represents the PWM PTC peripheral on JH71xx-based SoCs.
pub struct Pwm<PWM: PwmPeripheral> {
    periph: PWM,
}

impl<PWM: PwmPeripheral> Pwm<PWM> {
    /// Creates a new [Pwm] from a PWM peripheral.
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use jh71xx_hal::{pac, pwm};
    /// let dp = pac::Peripherals::take().unwrap();
    /// let _pwm = pwm::Pwm::new(dp.PWM);
    /// ```
    pub fn new(mut periph: PWM) -> Self {
        if periph.period() > MAX_PERIOD {
            periph.set_period(MAX_PERIOD);
        }
        Self { periph }
    }

    /// Gets the period of the [Pwm] peripheral.
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use jh71xx_hal::{pac, pwm};
    /// let dp = pac::Peripherals::take().unwrap();
    /// let pwm = pwm::Pwm::new(dp.PWM);
    /// let _period = pwm.period();
    /// ```
    pub fn period(&self) -> u16 {
        (self.periph.period() & 0xffff) as u16
    }

    /// Sets the period of the [Pwm] peripheral.
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use jh71xx_hal::{pac, pwm};
    /// let dp = pac::Peripherals::take().unwrap();
    /// let mut pwm = pwm::Pwm::new(dp.PWM);
    /// pwm.set_period(pwm::MAX_PERIOD as u16);
    /// ```
    pub fn set_period(&mut self, period: u16) {
        self.periph.set_period(period as u32);
    }

    /// Gets whether the [Pwm] peripheral is enabled.
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use jh71xx_hal::{pac, pwm};
    /// let dp = pac::Peripherals::take().unwrap();
    /// let pwm = pwm::Pwm::new(dp.PWM);
    /// if pwm.enabled() {
    ///     // do interesting PWM stuff
    /// }
    /// ```
    pub fn enabled(&self) -> bool {
        self.periph.enabled()
    }

    /// Sets whether the PWM peripheral is enabled.
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use jh71xx_hal::{pac, pwm};
    /// let dp = pac::Peripherals::take().unwrap();
    /// let mut pwm = pwm::Pwm::new(dp.PWM);
    /// if !pwm.enabled() {
    ///     pwm.enable(true);
    /// }
    /// ```
    pub fn enable(&mut self, val: bool) {
        self.periph.enable(val);
    }
}

impl<PWM: PwmPeripheral> ErrorType for Pwm<PWM> {
    type Error = Error;
}

impl<PWM: PwmPeripheral> SetDutyCycle for Pwm<PWM> {
    fn max_duty_cycle(&self) -> u16 {
        (self.periph.period() & 0xffff) as u16
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<()> {
        self.periph.set_duty(duty as u32);
        Ok(())
    }
}
