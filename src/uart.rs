//! UART peripheral access

use core::marker::PhantomData;

mod config;
mod error;
mod serial;

pub use config::*;
pub use error::*;
pub use serial::*;

/// Clock used by Dw_apb_uart: 50 MHz
pub const APB0: usize = 50_000_000;
/// Core clock oscillator: 24 MHz
pub const CLK_OSC: usize = 24_000_000;

/// Transaction timeout in microseconds.
pub const TIMEOUT_US: u64 = 1_000_000;

/// Represents UART TX functionality.
///
/// Inspired by `esp-hal` implementation: <https://github.com/esp-rs/esp-hal>
pub struct UartTx<'d, Block, T: Serial<Block>> {
    _serial: PhantomData<&'d mut T>,
    _block: PhantomData<Block>,
}

impl<'d, Block, T: Serial<Block>> UartTx<'d, Block, T> {
    fn new_inner() -> Self {
        Self {
            _serial: PhantomData,
            _block: PhantomData,
        }
    }

    /// Writes bytes over serial.
    ///
    /// Blocking function.
    ///
    /// Returns:
    ///
    /// - `Ok(written: usize)` on success, `written` bytes written to peripheral
    /// - `Err(Error)` on failure
    pub fn write_bytes(&mut self, data: &[u8]) -> Result<usize> {
        let count = data.len();

        data.iter()
            .try_for_each(|&c| nb::block!(self.write_byte(c)))?;

        Ok(count)
    }

    fn write_byte(&mut self, byte: u8) -> nb::Result<(), Error> {
        T::write_byte(byte)
    }

    fn flush(&mut self) -> nb::Result<(), Error> {
        T::flush()
    }
}

/// Represents UART RX functionality.
///
/// Based on the implementation in `esp-hal`: <https://github.com/esp-rs/esp-hal>
pub struct UartRx<'d, Block, T: Serial<Block>> {
    _serial: PhantomData<&'d mut T>,
    _block: PhantomData<Block>,
}

impl<'d, B, T: Serial<B>> UartRx<'d, B, T> {
    fn new_inner() -> Self {
        Self {
            _serial: PhantomData,
            _block: PhantomData,
        }
    }

    /// Reads bytes from the peripheral.
    ///
    /// Continues to read bytes while receive FIFO is full, or at least one byte is read.
    pub fn read_bytes(&mut self, buf: &mut [u8]) -> Result<usize> {
        let mut count = 0usize;

        for byte in buf.iter_mut() {
            match self.read_byte() {
                Ok(b) => {
                    *byte = b;
                    count = count.saturating_add(1);
                }
                Err(nb::Error::WouldBlock) => {
                    // Block until we have at least one byte
                    if count > 0 {
                        break;
                    }
                }
                Err(nb::Error::Other(e)) => return Err(e),
            }
        }

        Ok(count)
    }

    fn read_byte(&mut self) -> nb::Result<u8, Error> {
        T::read_byte()
    }
}

/// Represents a UART peripheral.
///
/// Based on the implementation in [`esp-hal`](https://github.com/esp-rs/esp-hal).
#[repr(C)]
pub struct Uart<'d, B, UART: Serial<B>> {
    tx: UartTx<'d, B, UART>,
    rx: UartRx<'d, B, UART>,
    timeout: u64,
    config: Config,
}

impl<'d, B, UART: Serial<B>> Uart<'d, B, UART> {
    /// Creates a new [Uart].
    ///
    /// Parameters:
    ///
    /// - `uart`: UART peripheral that implements the [Serial] trait.
    pub fn new(uart: UART) -> Self {
        Self::new_with_config(uart, TIMEOUT_US, Config::new())
    }

    /// Creates a new [Uart] from a custom configuration.
    ///
    /// Parameters:
    ///
    /// - `uart`: UART peripheral that implements the [Serial] trait.
    /// - `timeout`: time in microseconds before aborting transaction.
    /// - `config`: UART configuration parameters.
    pub fn new_with_config(mut uart: UART, timeout: u64, config: Config) -> Self {
        uart.setup(config).ok();

        Self {
            tx: UartTx::new_inner(),
            rx: UartRx::new_inner(),
            timeout,
            config,
        }
    }

    /// Splits the [Uart] into a transmitter and receiver
    pub fn split(self) -> (UartTx<'d, B, UART>, UartRx<'d, B, UART>) {
        (self.tx, self.rx)
    }

    /// Read a byte from the UART FIFO.
    pub fn read_byte(&mut self) -> Result<u8> {
        Ok(self.rx.read_byte()?)
    }

    /// Write a byte to the UART FIFO.
    pub fn write_byte(&mut self, byte: u8) -> Result<()> {
        Ok(self.tx.write_byte(byte)?)
    }

    /// Gets the timeout (in microseconds).
    pub const fn timeout(&self) -> u64 {
        self.timeout
    }

    /// Sets the timeout (in microseconds).
    ///
    /// **NOTE**: `timeout` must be greater than zero, no-op otherwise.
    pub fn set_timeout(&mut self, timeout: u64) {
        if timeout > 0 {
            self.timeout = timeout;
        }
    }

    /// Builder function that sets the timeout (in microseconds).
    ///
    /// **NOTE**: `timeout` must be greater than zero, no-op otherwise.
    pub fn with_timeout(mut self, timeout: u64) -> Self {
        self.set_timeout(timeout);
        self
    }

    /// Gets the [Config].
    pub const fn config(&self) -> Config {
        self.config
    }

    /// Sets the [Config].
    pub fn set_config(&mut self, config: Config) {
        self.config = config;
    }

    /// Builder function that sets the [Config].
    pub fn with_config(mut self, config: Config) -> Self {
        self.set_config(config);
        self
    }
}

impl<'d, B, UART: Serial<B>> io::ErrorType for Uart<'d, B, UART> {
    type Error = Error;
}

impl<'d, B, UART: Serial<B>> io::ErrorType for UartRx<'d, B, UART> {
    type Error = Error;
}

impl<'d, B, UART: Serial<B>> io::ErrorType for UartTx<'d, B, UART> {
    type Error = Error;
}

impl<'d, B, UART: Serial<B>> embedded_hal_nb::serial::ErrorType for Uart<'d, B, UART> {
    type Error = Error;
}

impl<'d, B, UART: Serial<B>> embedded_hal_nb::serial::ErrorType for UartRx<'d, B, UART> {
    type Error = Error;
}

impl<'d, B, UART: Serial<B>> embedded_hal_nb::serial::ErrorType for UartTx<'d, B, UART> {
    type Error = Error;
}

impl<'d, B, UART: Serial<B>> io::Read for Uart<'d, B, UART> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.rx.read_bytes(buf)
    }
}

impl<'d, B, UART: Serial<B>> io::Read for UartRx<'d, B, UART> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.read_bytes(buf)
    }
}

impl<'d, B, UART: Serial<B>> io::Write for Uart<'d, B, UART> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.tx.write_bytes(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.tx.flush()?;
        Ok(())
    }
}

impl<'d, B, UART: Serial<B>> io::Write for UartTx<'d, B, UART> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.write_bytes(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.flush()?;
        Ok(())
    }
}

impl<'d, B, UART: Serial<B>> embedded_hal_nb::serial::Read for Uart<'d, B, UART> {
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        Ok(self.rx.read_byte()?)
    }
}

impl<'d, B, UART: Serial<B>> embedded_hal_nb::serial::Read for UartRx<'d, B, UART> {
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        Ok(self.read_byte()?)
    }
}

impl<'d, B, UART: Serial<B>> embedded_hal_nb::serial::Write for Uart<'d, B, UART> {
    fn write(&mut self, val: u8) -> nb::Result<(), Self::Error> {
        Ok(self.tx.write_byte(val)?)
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        Ok(self.tx.flush()?)
    }
}

impl<'d, B, UART: Serial<B>> embedded_hal_nb::serial::Write for UartTx<'d, B, UART> {
    fn write(&mut self, val: u8) -> nb::Result<(), Self::Error> {
        Ok(self.write_byte(val)?)
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        Ok(self.flush()?)
    }
}
