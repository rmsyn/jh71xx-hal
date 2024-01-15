use crate::pac::{UART0, UART1, UART2, UART3, UART4, UART5};

use super::{Config, Error, Result};

/// Traits for access to a UART peripheral.
///
/// Provides abstractions over common actions for UART peripherals, like setup, reading, and
/// writing.
pub trait Serial {
    /// Performs setup initialization for the UART peripheral.
    fn setup(&mut self, config: Config) -> Result<()>;
    /// Reads a byte from the UART peripheral (blocking).
    fn read_byte() -> nb::Result<u8, Error>;
    /// Writes a byte to the UART peripheral (blocking).
    fn write_byte(byte: u8) -> nb::Result<(), Error>;
    /// Flushes the UART peripheral transmit buffer (blocking).
    fn flush() -> nb::Result<(), Error>;
}

// Convenience macro for implementing the [Serial] trait over a UART peripheral type.
//
// Abstracts register access to follow DRY principles.
macro_rules! impl_uart {
    ($uart:ident) => {
        impl $crate::uart::Serial for $uart {
            fn setup(&mut self, config: $crate::uart::Config) -> $crate::uart::Result<()> {
                // wait for UART0 to be idle
                while self.usr().read().busy().bit_is_set() {}

                // Set DLAB to make DLL and DLH registers accessible
                self.lcr().modify(|_, w| w.dlab().set_bit());

                // Set Divisor Latch Low and Divisor Latch High register values
                self.dll()
                    .write(|w| w.dll().variant(config.baud_rate.dll(config.clk_hz)));
                self.dlh()
                    .write(|w| w.dlh().variant(config.baud_rate.dlh(config.clk_hz)));

                // Clear DLAB to make RBR and THR registers accessible
                self.lcr().modify(|_, w| w.dlab().clear_bit());

                self.lcr().modify(|_, w| {
                    // Configure the data length
                    w.dls().variant(config.data_len as u8);

                    // Configure the number of stop bits
                    match config.stop {
                        $crate::uart::Stop::One => w.stop().clear_bit(),
                        $crate::uart::Stop::Two => w.stop().set_bit(),
                    };

                    // Configure the parity bits
                    match config.parity {
                        $crate::uart::Parity::None => w.pen().clear_bit(),
                        $crate::uart::Parity::Odd => {
                            w.pen().set_bit();
                            w.eps().clear_bit()
                        }
                        $crate::uart::Parity::Even => {
                            w.pen().set_bit();
                            w.eps().set_bit()
                        }
                    }
                });

                // Disable auto flow control: from `oreboot` startup
                self.mcr().modify(|_, w| w.afce().clear_bit());

                self.fcr().modify(|_, w| {
                    // Program FIFO enabled: from `oreboot` startup
                    w.fifoe().set_bit();
                    w.dmam().clear_bit();
                    // Trigger on the 8th byte
                    w.rt().variant(0b10);
                    // Reset the receiver and transmitter FIFOs: from `oreboot` startup
                    w.rfifor().set_bit();
                    w.xfifor().set_bit()
                });

                // Disable interrupts: from `oreboot` startup
                self.ier().modify(|_, w| w.ptime().clear_bit());

                Ok(())
            }

            fn read_byte() -> nb::Result<u8, Error> {
                // SAFETY: caller must ensure exclusive access to the UART peripheral
                let uart = unsafe { &*Self::ptr() };
                if uart.lsr().read().dr().bit_is_set() {
                    Ok(uart.rbr().read().rbr().bits())
                } else {
                    Err(nb::Error::WouldBlock)
                }
            }

            fn write_byte(byte: u8) -> nb::Result<(), Error> {
                // SAFETY: caller must ensure exclusive access to the UART peripheral
                let uart = unsafe { &*Self::ptr() };
                if uart.lsr().read().thre().bit_is_set() {
                    uart.thr().write(|w| w.thr().variant(byte));
                    Ok(())
                } else {
                    Err(nb::Error::WouldBlock)
                }
            }

            fn flush() -> nb::Result<(), Error> {
                // SAFETY: caller must ensure exclusive access to the UART peripheral
                let uart = unsafe { &*Self::ptr() };
                // Reset the receive and transmit FIFOs
                uart.fcr()
                    .modify(|_, w| w.rfifor().set_bit().xfifor().set_bit());
                Ok(())
            }
        }
    };
}

impl_uart!(UART0);
impl_uart!(UART1);
impl_uart!(UART2);
impl_uart!(UART3);
impl_uart!(UART4);
impl_uart!(UART5);
