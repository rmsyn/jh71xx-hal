use embedded_hal::spi::{ErrorType, SpiBus};

mod error;
mod peripheral;

pub use error::*;
pub use peripheral::*;

/// Represents an SPI peripheral on a JH71xx-based SoC.
#[repr(C)]
pub struct Spi<SPI: SpiPeripheral> {
    periph: SPI,
}

impl<SPI: SpiPeripheral> Spi<SPI> {
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
    /// let _spi = spi::Spi::new(dp.SPI0, spi::DataSize::Eight);
    /// ```
    pub fn new(mut periph: SPI, data_size: DataSize) -> Result<Self> {
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

impl<SPI: SpiPeripheral> ErrorType for Spi<SPI> {
    type Error = Error;
}

impl<SPI: SpiPeripheral> SpiBus<u8> for Spi<SPI> {
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

impl<SPI: SpiPeripheral> SpiBus<u16> for Spi<SPI> {
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
