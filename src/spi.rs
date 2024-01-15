//! SPI configuration and access.
//!
//! ## Examples
//!
//! ```no_run
//! use embedded_hal::spi::SpiBus;
//! use jh71xx_hal::{pac, spi};
//!
//! let dp = pac::Peripherals::take().unwrap();
//!
//! // 8-bit transactions
//! let mut spi0 = spi::Spi::<pac::SPI0, 8>::new(dp.SPI0).unwrap();
//! let mut read_buf = [0u8; 1];
//! let write_buf = [0u8; 1];
//!
//! // Read and write as separate transactions
//! spi0.read(read_buf.as_mut()).unwrap();
//! spi0.write(write_buf.as_ref()).unwrap();
//!
//! // Read and write in the same call
//! spi0.transfer(read_buf.as_mut(), write_buf.as_ref()).unwrap();
//! // Write and read from the same buffer
//! // NOTE: writes happen first, since the read overwrites the buffer
//! spi0.transfer_in_place(read_buf.as_mut()).unwrap();
//!
//! // Flushes read/write FIFOs, and waits for peripheral to become idle
//! spi0.flush().unwrap();
//!
//! // 16-bit transactions
//! let mut spi1 = spi::Spi::<pac::SPI1, 16>::new(dp.SPI1).unwrap();
//! let mut read_buf = [0u16; 1];
//! let write_buf = [0u16; 1];
//!
//! // Read and write as separate transactions
//! spi1.read(read_buf.as_mut()).unwrap();
//! spi1.write(write_buf.as_ref()).unwrap();
//!
//! // Read and write in the same call
//! spi1.transfer(read_buf.as_mut(), write_buf.as_ref()).unwrap();
//! // Write and read from the same buffer
//! // NOTE: writes happen first, since the read overwrites the buffer
//! spi1.transfer_in_place(read_buf.as_mut()).unwrap();
//!
//! // Flushes read/write FIFOs, and waits for peripheral to become idle
//! spi1.flush().unwrap();
//! ```
//!
//! ### WIP
//!
//! Currently, only 8- and 16-bit transfers are supported. The peripheral in the SoC supports 4- to 16-bit transfers.
//!
//! TBD: should the interface support:
//!
//! - packed data transfers for efficiency (but increased complexity)
//! - unpacked transfers for simplicity (but reduced efficiency)
//!
//! The [ARM pl022 SSP SPI](https://documentation-service.arm.com/static/5e8e3b2afd977155116a92f7&rut=3d45d778b3f2b62fe659ebfb50905914d913d289f017585fb1c8e07383ea508a) peripheral also supports "Slave" mode, which is outside the `embedded-hal` traits, but could still be useful to `jh71xx-hal` users.
//!
//! Similarly, the peripheral supports the Texas Instruments Synchronous Serial and Microwire serial frame formats (currently unsupported).

use embedded_hal::spi::{ErrorType, SpiBus};

mod error;
mod peripheral;

pub use error::*;
pub use peripheral::*;

/// Represents an SPI peripheral on a JH71xx-based SoC.
#[repr(C)]
pub struct Spi<SPI: SpiPeripheral, const WORD: u8> {
    periph: SPI,
}

impl<SPI: SpiPeripheral, const WORD: u8> Spi<SPI, WORD> {
    /// Creates a new [Spi] from an SPI peripheral.
    ///
    /// Parameters:
    ///
    /// - `data_size`: [DataSize] for transfers. Currently, only 8-bit and 16-bit supported.
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use jh71xx_hal::{pac, spi};
    /// let dp = pac::Peripherals::take().unwrap();
    /// let _spi = spi::Spi::<pac::SPI0, 8>::new(dp.SPI0);
    /// ```
    pub fn new(mut periph: SPI) -> Result<Self> {
        let data_size = DataSize::from(WORD);
        match data_size {
            DataSize::Eight | DataSize::Sixteen => {
                periph.set_dss(data_size);
                periph.set_ms(ModeSelect::Master);
                periph.set_frf(FrameFormat::Spi);
                Ok(Self { periph })
            }
            _ => Err(Error::DataSize(data_size)),
        }
    }

    /// Splits the [Spi] back into the inner peripheral type.
    pub fn split(self) -> SPI {
        self.periph
    }
}

impl<SPI: SpiPeripheral, const WORD: u8> ErrorType for Spi<SPI, WORD> {
    type Error = Error;
}

impl<SPI: SpiPeripheral> SpiBus<u8> for Spi<SPI, 8> {
    fn read(&mut self, words: &mut [u8]) -> Result<()> {
        for word in words.iter_mut() {
            // Spin until receive FIFO is full
            while !self.periph.rff() || self.periph.bsy() {
                // Check for receive timeout interrupt (after masking)
                if self.periph.rtmis() {
                    self.periph.rtic(true);
                    return Err(Error::Timeout);
                // Check for receive overrun interrupt (after masking)
                } else if self.periph.rormis() {
                    self.periph.roric(true);
                    return Err(Error::Overrun);
                }
            }
            // FIXME: support 4-7 bit data sizes
            *word = (self.periph.data() & 0xff) as u8;
        }
        Ok(())
    }

    fn write(&mut self, words: &[u8]) -> Result<()> {
        for word in words.iter() {
            while !self.periph.tfe() {}
            self.periph.set_data(*word);
        }
        Ok(())
    }

    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<()> {
        let rlen = read.len();
        let wlen = write.len();
        let len = core::cmp::min(rlen, wlen);

        for i in 0..len {
            self.read(&mut read[i..i + 1])?;
            self.write(&write[i..i + 1])?;
        }

        if rlen > len {
            self.read(&mut read[len..])
        } else if wlen > len {
            self.write(&write[len..])
        } else {
            Ok(())
        }
    }

    fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<()> {
        for i in 0..words.len() {
            self.write(&words[i..i + 1])?;
            self.read(&mut words[i..i + 1])?;
        }
        Ok(())
    }

    fn flush(&mut self) -> Result<()> {
        // clear receiver interrupts
        self.periph.roric(true);
        self.periph.rtic(true);

        // spin while FIFOs are not empty, and/or the peripheral is busy
        while !self.periph.tfe() || self.periph.rne() || self.periph.bsy() {
            core::hint::spin_loop();
        }

        Ok(())
    }
}

impl<SPI: SpiPeripheral> SpiBus<u16> for Spi<SPI, 16> {
    fn read(&mut self, words: &mut [u16]) -> Result<()> {
        for word in words.iter_mut() {
            // Spin until receive FIFO is full
            while !self.periph.rff() || self.periph.bsy() {
                // Check for receive timeout interrupt (after masking)
                if self.periph.rtmis() {
                    self.periph.rtic(true);
                    return Err(Error::Timeout);
                // Check for receive overrun interrupt (after masking)
                } else if self.periph.rormis() {
                    self.periph.roric(true);
                    return Err(Error::Overrun);
                }
            }
            // FIXME: support 4-15 bit data sizes
            *word = self.periph.data();
        }
        Ok(())
    }

    fn write(&mut self, words: &[u16]) -> Result<()> {
        for word in words.iter() {
            while !self.periph.tfe() {}
            self.periph.set_data(*word);
        }
        Ok(())
    }

    fn transfer(&mut self, read: &mut [u16], write: &[u16]) -> Result<()> {
        let rlen = read.len();
        let wlen = write.len();
        let len = core::cmp::min(rlen, wlen);

        for i in 0..len {
            self.read(&mut read[i..i + 1])?;
            self.write(&write[i..i + 1])?;
        }

        if rlen > len {
            self.read(&mut read[len..])
        } else if wlen > len {
            self.write(&write[len..])
        } else {
            Ok(())
        }
    }

    fn transfer_in_place(&mut self, words: &mut [u16]) -> Result<()> {
        for i in 0..words.len() {
            self.write(&words[i..i + 1])?;
            self.read(&mut words[i..i + 1])?;
        }
        Ok(())
    }

    fn flush(&mut self) -> Result<()> {
        // clear receiver interrupts
        self.periph.roric(true);
        self.periph.rtic(true);

        // spin while FIFOs are not empty, and/or the peripheral is busy
        while !self.periph.tfe() || self.periph.rne() || self.periph.bsy() {}

        Ok(())
    }
}

impl<SPI: SpiPeripheral> TryFrom<Spi<SPI, 8>> for Spi<SPI, 16> {
    type Error = Error;

    fn try_from(val: Spi<SPI, 8>) -> Result<Self> {
        Self::new(val.split())
    }
}

impl<SPI: SpiPeripheral> TryFrom<Spi<SPI, 16>> for Spi<SPI, 8> {
    type Error = Error;

    fn try_from(val: Spi<SPI, 16>) -> Result<Self> {
        Self::new(val.split())
    }
}
