use super::APB0;

/// Fixed divisor constant multiplier.
///
/// The baud rate is calculated as: `CLK_HZ` / (`FIXED_DIV` * `BAUD_DIV`)
///
/// `BAUD_DIV` is encoded in the `DLL` and `DLH` registers.
pub const FIXED_DIV: usize = 16;

/// Values for selecting the data length (in bits) via the DLS (Data Length Select).
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum DataLength {
    Five = 0b00,
    Six = 0b01,
    Seven = 0b10,
    #[default]
    Eight = 0b11,
}

impl DataLength {
    /// Creates a new [DataLength].
    pub const fn new() -> Self {
        Self::Eight
    }
}

/// Parity select values for enabling/disabling parity bits.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Parity {
    /// Send an odd number of logical `1`s as parity bits.
    Odd = 0,
    /// Send an even number of logical `1`s as parity bits.
    Even = 1,
    /// Disable parity bits.
    #[default]
    None,
}

impl Parity {
    /// Creates a new [Parity].
    pub const fn new() -> Self {
        Self::None
    }
}

/// Configure the number of stop bits.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Stop {
    /// Send one stop bit to signal the end of transmission.
    #[default]
    One = 0,
    /// Send 1.5 or 2 stop bits to signal the end of transmission.
    Two = 1,
}

impl Stop {
    /// Creates a new [Stop].
    pub const fn new() -> Self {
        Self::One
    }
}

/// Represents baud rate divisior arguments to setup the UART peripheral.
///
/// The baud rate divisor is split into two 8-bit registers: DLL and DLM.
#[repr(usize)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BaudRate {
    B1200 = 1_200,
    B2400 = 2_400,
    B4800 = 4_800,
    B9600 = 9_600,
    B19200 = 19_200,
    B38400 = 38_400,
    B57600 = 57_600,
    #[default]
    B115200 = 115_200,
}

impl BaudRate {
    /// Creates a new [BaudRate].
    pub const fn new() -> Self {
        Self::B115200
    }

    /// Gets the DLL divisor value.
    pub const fn dll(&self, clk_hz: usize) -> u8 {
        self.baud_divisor(clk_hz) as u8
    }

    /// Gets the DLH divisor value.
    pub const fn dlh(&self, clk_hz: usize) -> u8 {
        ((self.baud_divisor(clk_hz) & 0xff00) >> 8) as u8
    }

    /// Gets the baud divisor value.
    pub const fn baud_divisor(&self, clk_hz: usize) -> u16 {
        clk_hz
            .saturating_div(FIXED_DIV)
            .saturating_div(*self as usize) as u16
    }
}

/// Configuration settings for UART peripherals.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Config {
    pub data_len: DataLength,
    pub stop: Stop,
    pub parity: Parity,
    pub baud_rate: BaudRate,
    pub clk_hz: usize,
}

impl Config {
    /// Creates a new [Config].
    pub const fn new() -> Self {
        Self {
            data_len: DataLength::new(),
            stop: Stop::new(),
            parity: Parity::new(),
            baud_rate: BaudRate::new(),
            clk_hz: APB0,
        }
    }
}
