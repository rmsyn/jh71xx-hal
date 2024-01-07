use super::I2cSpeedMode;

/// I2C timing information
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct I2cTimings {
    bus_freq_hz: I2cSpeedMode,
    scl_rise_ns: u32,
    scl_fall_ns: u32,
    scl_int_delay_ns: u32,
    sda_fall_ns: u32,
    sda_hold_ns: u32,
    digital_filter_width_ns: u32,
    analog_filter_cutoff_freq_hz: u32,
}

impl I2cTimings {
    /// Creates a new [I2cTimings].
    pub const fn new() -> Self {
        Self {
            bus_freq_hz: I2cSpeedMode::new(),
            scl_rise_ns: 0,
            scl_fall_ns: 0,
            scl_int_delay_ns: 0,
            sda_fall_ns: 0,
            sda_hold_ns: 0,
            digital_filter_width_ns: 0,
            analog_filter_cutoff_freq_hz: 0,
        }
    }

    /// Gets the bus frequency in Hz.
    pub const fn bus_freq_hz(&self) -> I2cSpeedMode {
        self.bus_freq_hz
    }

    /// Sets the bus frequency in Hz.
    pub fn set_bus_freq_hz<S: Into<I2cSpeedMode>>(&mut self, val: S) {
        self.bus_freq_hz = val.into();
    }

    /// Builder function that sets the bus frequency in Hz.
    pub fn with_bus_freq_hz<S: Into<I2cSpeedMode>>(mut self, val: S) -> Self {
        self.set_bus_freq_hz(val);
        self
    }

    /// Gets the time SCL signal takes to rise in ns; t(r) in the I2C specification.
    pub const fn scl_rise_ns(&self) -> u32 {
        self.scl_rise_ns
    }

    /// Sets the time SCL signal takes to rise in ns; t(r) in the I2C specification.
    pub fn set_scl_rise_ns(&mut self, val: u32) {
        self.scl_rise_ns = val;
    }

    /// Builder function that sets the time SCL signal takes to rise in ns; t(r) in the I2C specification.
    pub fn with_scl_rise_ns(mut self, val: u32) -> Self {
        self.set_scl_rise_ns(val);
        self
    }

    /// Gets the time SCL signal takes to fall in ns; t(f) in the I2C specification.
    pub const fn scl_fall_ns(&self) -> u32 {
        self.scl_fall_ns
    }

    /// Sets the time SCL signal takes to fall in ns; t(f) in the I2C specification.
    pub fn set_scl_fall_ns(&mut self, val: u32) {
        self.scl_fall_ns = val;
    }

    /// Builder function that sets the time SCL signal takes to fall in ns; t(f) in the I2C specification.
    pub fn with_scl_fall_ns(mut self, val: u32) -> Self {
        self.set_scl_fall_ns(val);
        self
    }

    /// Gets the time IP core additionally needs to setup SCL in ns.
    pub const fn scl_int_delay_ns(&self) -> u32 {
        self.scl_int_delay_ns
    }

    /// Sets the time SCL signal takes to fall in ns; t(f) in the I2C specification.
    pub fn set_scl_int_delay_ns(&mut self, val: u32) {
        self.scl_int_delay_ns = val;
    }

    /// Builder function that sets the time SCL signal takes to fall in ns; t(f) in the I2C specification.
    pub fn with_scl_int_delay_ns(mut self, val: u32) -> Self {
        self.set_scl_int_delay_ns(val);
        self
    }

    /// Gets the time SDA signl takes to fall in ns; t(f) in the I2C specification.
    pub const fn sda_fall_ns(&self) -> u32 {
        self.sda_fall_ns
    }

    /// Sets the time SDA signl takes to fall in ns; t(f) in the I2C specification.
    pub fn set_sda_fall_ns(&mut self, val: u32) {
        self.sda_fall_ns = val;
    }

    /// Builder function that sets the time SDA signl takes to fall in ns; t(f) in the I2C specification.
    pub fn with_sda_fall_ns(mut self, val: u32) -> Self {
        self.set_sda_fall_ns(val);
        self
    }

    /// Gets the time IP core additionally needs to hold SDA in ns.
    pub const fn sda_hold_ns(&self) -> u32 {
        self.sda_hold_ns
    }

    /// Sets the time IP core additionally needs to hold SDA in ns.
    pub fn set_sda_hold_ns(&mut self, val: u32) {
        self.sda_hold_ns = val;
    }

    /// Builder function that sets the time IP core additionally needs to hold SDA in ns.
    pub fn with_sda_hold_ns(mut self, val: u32) -> Self {
        self.set_sda_hold_ns(val);
        self
    }

    /// Gets the width in ns of spikes on I2C lines that the IP core digital filter can filter out.
    pub const fn digital_filter_width_ns(&self) -> u32 {
        self.digital_filter_width_ns
    }

    /// Sets the width in ns of spikes on I2C lines that the IP core digital filter can filter out.
    pub fn set_digital_filter_width_ns(&mut self, val: u32) {
        self.digital_filter_width_ns = val;
    }

    /// Builder function that sets the width in ns of spikes on I2C lines that the IP core digital filter can filter out.
    pub fn with_digital_filter_width_ns(mut self, val: u32) -> Self {
        self.set_digital_filter_width_ns(val);
        self
    }

    /// Gets the threshold frequency for the low pass IP core analog filter.
    pub const fn analog_filter_cutoff_freq_hz(&self) -> u32 {
        self.analog_filter_cutoff_freq_hz
    }

    /// Sets the threshold frequency for the low pass IP core analog filter.
    pub fn set_analog_filter_cutoff_freq_hz(&mut self, val: u32) {
        self.analog_filter_cutoff_freq_hz = val;
    }

    /// Builder function that sets the threshold frequency for the low pass IP core analog filter.
    pub fn with_analog_filter_cutoff_freq_hz(mut self, val: u32) -> Self {
        self.set_analog_filter_cutoff_freq_hz(val);
        self
    }
}
