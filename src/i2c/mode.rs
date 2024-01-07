use core::fmt;

pub const I2C_SPEED_MODE_STD: u32 = 100_000;
pub const I2C_SPEED_MODE_FAST: u32 = 400_000;
pub const I2C_SPEED_MODE_FAST_PLUS: u32 = 1_000_000;
pub const I2C_SPEED_MODE_TURBO: u32 = 1_400_000;
pub const I2C_SPEED_MODE_HIGH: u32 = 3_400_000;
pub const I2C_SPEED_MODE_ULTRA_FAST: u32 = 5_000_000;

pub const I2C_OP_MODE_MASTER: u32 = 0;
pub const I2C_OP_MODE_SLAVE: u32 = 1;

/// I2C Frequency Modes
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum I2cSpeedMode {
    #[default]
    Standard = I2C_SPEED_MODE_STD,
    Fast = I2C_SPEED_MODE_FAST,
    FastPlus = I2C_SPEED_MODE_FAST_PLUS,
    Turbo = I2C_SPEED_MODE_TURBO,
    High = I2C_SPEED_MODE_HIGH,
    UltraFast = I2C_SPEED_MODE_ULTRA_FAST,
}

impl I2cSpeedMode {
    /// Creates a new [I2cSpeedMode].
    pub const fn new() -> Self {
        Self::Standard
    }

    /// Creates a new [I2cSpeedMode] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        match val {
            I2C_SPEED_MODE_STD => Self::Standard,
            I2C_SPEED_MODE_FAST => Self::Fast,
            I2C_SPEED_MODE_FAST_PLUS => Self::FastPlus,
            I2C_SPEED_MODE_TURBO => Self::Turbo,
            I2C_SPEED_MODE_HIGH => Self::High,
            I2C_SPEED_MODE_ULTRA_FAST => Self::UltraFast,
            _ => Self::Standard,
        }
    }
}

impl From<I2cSpeedMode> for u32 {
    fn from(val: I2cSpeedMode) -> Self {
        val as u32
    }
}

impl From<&I2cSpeedMode> for u32 {
    fn from(val: &I2cSpeedMode) -> Self {
        (*val).into()
    }
}

impl From<u32> for I2cSpeedMode {
    fn from(val: u32) -> Self {
        Self::create(val)
    }
}

impl From<&I2cSpeedMode> for &'static str {
    fn from(val: &I2cSpeedMode) -> Self {
        match val {
            I2cSpeedMode::Standard => "standard",
            I2cSpeedMode::Fast => "fast",
            I2cSpeedMode::FastPlus => "fast plus",
            I2cSpeedMode::Turbo => "turbo",
            I2cSpeedMode::High => "high",
            I2cSpeedMode::UltraFast => "ultra fast",
        }
    }
}

impl From<I2cSpeedMode> for &'static str {
    fn from(val: I2cSpeedMode) -> Self {
        (&val).into()
    }
}

impl fmt::Display for I2cSpeedMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

/// I2C Operation Mode
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum I2cOpMode {
    #[default]
    Master = I2C_OP_MODE_MASTER,
    Slave = I2C_OP_MODE_SLAVE,
}

impl I2cOpMode {
    /// Creates a new [I2cOpMode].
    pub const fn new() -> Self {
        Self::Master
    }

    /// Creates a new [I2cOpMode] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        match val {
            I2C_OP_MODE_MASTER => Self::Master,
            I2C_OP_MODE_SLAVE => Self::Slave,
            _ => Self::Master,
        }
    }
}

impl From<I2cOpMode> for u32 {
    fn from(val: I2cOpMode) -> Self {
        val as u32
    }
}

impl From<&I2cOpMode> for u32 {
    fn from(val: &I2cOpMode) -> Self {
        (*val).into()
    }
}

impl From<u32> for I2cOpMode {
    fn from(val: u32) -> Self {
        Self::create(val)
    }
}

impl From<&I2cOpMode> for &'static str {
    fn from(val: &I2cOpMode) -> Self {
        match val {
            I2cOpMode::Master => "master",
            I2cOpMode::Slave => "slave",
        }
    }
}

impl From<I2cOpMode> for &'static str {
    fn from(val: I2cOpMode) -> Self {
        (&val).into()
    }
}

impl fmt::Display for I2cOpMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}
