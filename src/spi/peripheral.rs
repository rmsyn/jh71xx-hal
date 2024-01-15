use core::fmt;

use pac::{SPI0, SPI1, SPI2, SPI3, SPI4, SPI5, SPI6};

/// Represents the data word size (in bits) of the FIFO buffers.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum DataSize {
    Reserved = 0b0000,
    Four = 0b0011,
    Five = 0b0100,
    Six = 0b0101,
    Seven = 0b0110,
    #[default]
    Eight = 0b0111,
    Nine = 0b1000,
    Ten = 0b1001,
    Eleven = 0b1010,
    Twelve = 0b1011,
    Thirteen = 0b1100,
    Fourteen = 0b1101,
    Fifteen = 0b1110,
    Sixteen = 0b1111,
}

impl From<&DataSize> for &'static str {
    fn from(val: &DataSize) -> Self {
        match val {
            DataSize::Reserved => "reserved",
            DataSize::Four => "4 bits",
            DataSize::Five => "5 bits",
            DataSize::Six => "6 bits",
            DataSize::Seven => "7 bits",
            DataSize::Eight => "8 bits",
            DataSize::Nine => "9 bits",
            DataSize::Ten => "10 bits",
            DataSize::Eleven => "11 bits",
            DataSize::Twelve => "12 bits",
            DataSize::Thirteen => "13 bits",
            DataSize::Fourteen => "14 bits",
            DataSize::Fifteen => "15 bits",
            DataSize::Sixteen => "16 bits",
        }
    }
}

impl From<DataSize> for &'static str {
    fn from(val: DataSize) -> Self {
        (&val).into()
    }
}

impl From<DataSize> for u8 {
    fn from(val: DataSize) -> Self {
        val as u8
    }
}

impl From<&DataSize> for u8 {
    fn from(val: &DataSize) -> Self {
        (*val).into()
    }
}

impl From<u8> for DataSize {
    fn from(val: u8) -> Self {
        match val {
            0b0011 => Self::Four,
            0b0100 => Self::Five,
            0b0101 => Self::Six,
            0b0110 => Self::Seven,
            0b0111 => Self::Eight,
            0b1000 => Self::Nine,
            0b1001 => Self::Ten,
            0b1010 => Self::Eleven,
            0b1011 => Self::Twelve,
            0b1100 => Self::Thirteen,
            0b1101 => Self::Fourteen,
            0b1110 => Self::Fifteen,
            0b1111 => Self::Sixteen,
            _ => Self::Reserved,
        }
    }
}

impl fmt::Display for DataSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&str>::from(self))
    }
}

/// Represents the `SSPCLKOUT` clock polarity settings.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ClockPolarity {
    #[default]
    Low = 0b0,
    High = 0b1,
}

impl From<bool> for ClockPolarity {
    fn from(val: bool) -> Self {
        match val {
            false => Self::Low,
            true => Self::High,
        }
    }
}

/// Represents the `SSPCLKOUT` clock phase settings.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ClockPhase {
    #[default]
    Low = 0b0,
    High = 0b1,
}

impl From<bool> for ClockPhase {
    fn from(val: bool) -> Self {
        match val {
            false => Self::Low,
            true => Self::High,
        }
    }
}

/// Represents the data frame format.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FrameFormat {
    /// Motorola SPI frame format.
    #[default]
    Spi = 0b00,
    /// Texas Instruments Synchronous Serial frame format.
    SyncSerial = 0b01,
    /// National Microwire frame format.
    Microwire = 0b10,
    Reserved = 0b11,
}

impl From<u8> for FrameFormat {
    fn from(val: u8) -> Self {
        match val {
            0b00 => Self::Spi,
            0b01 => Self::SyncSerial,
            0b10 => Self::Microwire,
            _ => Self::Reserved,
        }
    }
}

impl From<FrameFormat> for u8 {
    fn from(val: FrameFormat) -> Self {
        val as u8
    }
}

impl From<&FrameFormat> for u8 {
    fn from(val: &FrameFormat) -> Self {
        (*val).into()
    }
}

/// Selects the configured mode of the SSP SPI peripheral.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ModeSelect {
    #[default]
    Master = 0,
    Slave = 1,
}

impl From<bool> for ModeSelect {
    fn from(val: bool) -> Self {
        match val {
            false => Self::Master,
            true => Self::Slave,
        }
    }
}

/// Represents the `SSPCLKOUT` prescale divisor.
///
/// The valid range is `[2:254]`.
///
/// The cycle frequency is calculated as:
///
/// ```no_build,no_run
/// F[sspclkout] / (CPSDVSR * (1 + SCR))
/// ```
///
/// Where `CPSDVSR` is an even value from `[2:254]`, programmed through the `SSPCSR` register,
/// and `SCR` is a value from `[0:255]`.
pub struct PrescaleDivisor(u8);

impl From<u8> for PrescaleDivisor {
    fn from(val: u8) -> Self {
        match val {
            0 | 1 => Self(2),
            2..=254 if val % 2 == 0 => Self(val),
            3..=253 if val % 2 != 0 => Self(val + 1),
            255 => Self(254),
            _ => Self(2),
        }
    }
}

impl From<PrescaleDivisor> for u8 {
    fn from(val: PrescaleDivisor) -> Self {
        val.0
    }
}

impl From<&PrescaleDivisor> for u8 {
    fn from(val: &PrescaleDivisor) -> Self {
        val.0
    }
}

/// Represents the interrupt mask settings.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum InterruptMask {
    #[default]
    Masked = 0,
    NotMasked = 1,
}

impl From<bool> for InterruptMask {
    fn from(val: bool) -> Self {
        match val {
            false => Self::Masked,
            true => Self::NotMasked,
        }
    }
}

/// High-level, safe functions needed to access low-level SSP SPI registers.
pub trait SpiPeripheral {
    /// Gets the [DataSize] selected for SPI transfers.
    fn dss(&self) -> DataSize;
    /// Sets the [DataSize] selected for SPI transfers.
    fn set_dss(&mut self, val: DataSize);

    /// Gets the [FrameFormat] protocol selected for SPI transfers.
    fn frf(&self) -> FrameFormat;
    /// Sets the [FrameFormat] protocol selected for SPI transfers.
    fn set_frf(&mut self, val: FrameFormat);

    /// Gets the `SSPCLKOUT` [ClockPolarity].
    fn spo(&self) -> ClockPolarity;
    /// Sets the `SSPCLKOUT` [ClockPolarity].
    fn set_spo(&mut self, val: ClockPolarity);

    /// Gets the `SSPCLKOUT` [ClockPhase].
    fn sph(&self) -> ClockPhase;
    /// Sets the `SSPCLKOUT` [ClockPhase].
    fn set_sph(&mut self, val: ClockPhase);

    /// Gets the `SSPCLKOUT` serial clock rate.
    ///
    /// The frequency is calculated as:
    ///
    /// ```no_build,no_run
    /// F[sspclkout] / (CPSDVR * (1 + SCR))
    /// ```
    ///
    /// Where `CPSDVSR` is an even value from `[2:254]`, programmed through the `SSPCSR` register,
    /// and `SCR` is a value from `[0:255]`.
    fn scr(&self) -> u8;
    /// Sets the `SSPCLKOUT` serial clock rate.
    ///
    /// The frequency is calculated as:
    ///
    /// ```no_build,no_run
    /// F[sspclkout] / (CPSDVSR * (1 + SCR))
    /// ```
    ///
    /// Where `CPSDVSR` is an even value from `[2:254]`, programmed through the `SSPCSR` register,
    /// and `SCR` is a value from `[0:255]`.
    fn set_scr(&mut self, val: u8);

    /// Gets the [ModeSelect] configuration for the SPI peripheral.
    fn ms(&self) -> ModeSelect;
    /// Sets the [ModeSelect] configuration for the SPI peripheral.
    fn set_ms(&mut self, val: ModeSelect);

    /// Sets the `SSPCLKOUT` [PrescaleDivisor].
    ///
    /// The frequency is calculated as:
    ///
    /// ```no_build,no_run
    /// F[sspclkout] / (CPSDVSR * (1 + SCR))
    /// ```
    ///
    /// Where `CPSDVSR` is an even value from `[2:254]`, programmed through the `SSPCSR` register,
    /// and `SCR` is a value from `[0:255]`.
    fn cpsdvsr(&self) -> PrescaleDivisor;
    /// Sets the `SSPCLKOUT` [PrescaleDivisor].
    ///
    /// The frequency is calculated as:
    ///
    /// ```no_build,no_run
    /// F[sspclkout] / (CPSDVSR * (1 + SCR))
    /// ```
    ///
    /// Where `CPSDVSR` is an even value from `[2:254]`, programmed through the `SSPCSR` register,
    /// and `SCR` is a value from `[0:255]`.
    fn set_cpsdvsr(&mut self, val: PrescaleDivisor);

    /// Gets the data from the receive FIFO.
    ///
    /// The receive logic automatically right-justifies (LSB starts at bit zero).
    fn data(&self) -> u16;
    /// Sets the data from the transmit FIFO.
    ///
    /// If the [DataSize] is set to less than 16-bits, the data must be right-justified (LSB moved
    /// to bit zero). Any high-order bits above the configured [DataSize] will be ignored.
    fn set_data<D: Into<u16>>(&mut self, val: D);

    /// Clears the `SSPRORINTR` (read-overrun) interrupt.
    ///
    /// A value of `true` clears the interrupt, `false` is a no-op.
    fn roric(&mut self, val: bool);

    /// Clears the `SSPRTINTR` (read-timeout) interrupt.
    ///
    /// A value of `true` clears the interrupt, `false` is a no-op.
    fn rtic(&mut self, val: bool);

    /// Gets the receive overrun [InterruptMask] setting.
    fn rorim(&self) -> InterruptMask;
    /// Sets the receive overrun [InterruptMask] setting.
    fn set_rorim(&mut self, val: InterruptMask);

    /// Gets the receive timeout [InterruptMask] setting.
    fn rtim(&self) -> InterruptMask;
    /// Sets the receive timeout [InterruptMask] setting.
    fn set_rtim(&mut self, val: InterruptMask);

    /// Gets the receive FIFO half-full or less [InterruptMask] setting.
    fn rxim(&self) -> InterruptMask;
    /// Sets the receive FIFO half-full or less [InterruptMask] setting.
    fn set_rxim(&mut self, val: InterruptMask);

    /// Gets the transmit FIFO half-full or less [InterruptMask] setting.
    fn txim(&self) -> InterruptMask;
    /// Sets the transmit FIFO half-full or less [InterruptMask] setting.
    fn set_txim(&mut self, val: InterruptMask);

    /// Gets the receive overrun interrupt status (after masking).
    fn rormis(&self) -> bool;
    /// Gets the receive timeout interrupt status (after masking).
    fn rtmis(&self) -> bool;
    /// Gets the receive FIFO interrupt status (after masking).
    fn rxmis(&self) -> bool;
    /// Gets the transmit FIFO interrupt status (after masking).
    fn txmis(&self) -> bool;

    /// Gets the receive overrun interrupt status (before masking).
    fn rorris(&self) -> bool;
    /// Gets the receive timeout interrupt status (before masking).
    fn rtris(&self) -> bool;
    /// Gets the receive FIFO interrupt status (before masking).
    fn rxris(&self) -> bool;
    /// Gets the transmit FIFO interrupt status (before masking).
    fn txris(&self) -> bool;

    /// Gets whether the transmit FIFO is empty.
    fn tfe(&self) -> bool;
    /// Gets whether the transmit FIFO is not full.
    fn tnf(&self) -> bool;
    /// Gets whether the receive FIFO is not empty.
    fn rne(&self) -> bool;
    /// Gets whether the receive FIFO is full.
    fn rff(&self) -> bool;
    /// Gets whether the SSP peripheral is busy.
    fn bsy(&self) -> bool;
}

macro_rules! impl_spi_peripheral {
    ($spi:ident) => {
        impl $crate::spi::SpiPeripheral for $spi {
            fn dss(&self) -> $crate::spi::DataSize {
                self.ssp_cr0().read().dss().bits().into()
            }
            fn set_dss(&mut self, val: $crate::spi::DataSize) {
                self.ssp_cr0().modify(|_, w| w.dss().variant(val.into()));
            }

            fn frf(&self) -> $crate::spi::FrameFormat {
                self.ssp_cr0().read().frf().bits().into()
            }
            fn set_frf(&mut self, val: $crate::spi::FrameFormat) {
                self.ssp_cr0().modify(|_, w| w.frf().variant(val.into()));
            }

            fn spo(&self) -> $crate::spi::ClockPolarity {
                self.ssp_cr0().read().spo().bit_is_set().into()
            }
            fn set_spo(&mut self, val: $crate::spi::ClockPolarity) {
                self.ssp_cr0().modify(|_, w| match val {
                    $crate::spi::ClockPolarity::Low => w.spo().clear_bit(),
                    $crate::spi::ClockPolarity::High => w.spo().set_bit(),
                })
            }

            fn sph(&self) -> $crate::spi::ClockPhase {
                self.ssp_cr0().read().sph().bit_is_set().into()
            }
            fn set_sph(&mut self, val: $crate::spi::ClockPhase) {
                self.ssp_cr0().modify(|_, w| match val {
                    $crate::spi::ClockPhase::Low => w.sph().clear_bit(),
                    $crate::spi::ClockPhase::High => w.sph().set_bit(),
                })
            }

            fn scr(&self) -> u8 {
                self.ssp_cr0().read().scr().bits()
            }
            fn set_scr(&mut self, val: u8) {
                self.ssp_cr0().modify(|_, w| w.scr().variant(val));
            }

            fn ms(&self) -> $crate::spi::ModeSelect {
                self.ssp_cr1().read().ms().bit_is_set().into()
            }
            fn set_ms(&mut self, val: $crate::spi::ModeSelect) {
                self.ssp_cr1().modify(|_, w| match val {
                    $crate::spi::ModeSelect::Master => w.ms().clear_bit(),
                    $crate::spi::ModeSelect::Slave => w.ms().set_bit(),
                });
            }

            fn cpsdvsr(&self) -> $crate::spi::PrescaleDivisor {
                self.ssp_cpsr().read().cpsdvsr().bits().into()
            }
            fn set_cpsdvsr(&mut self, val: $crate::spi::PrescaleDivisor) {
                self.ssp_cpsr()
                    .modify(|_, w| w.cpsdvsr().variant(val.into()));
            }

            fn data(&self) -> u16 {
                self.ssp_dr().read().data().bits()
            }
            fn set_data<D: Into<u16>>(&mut self, val: D) {
                self.ssp_dr().modify(|_, w| w.data().variant(val.into()));
            }

            fn roric(&mut self, val: bool) {
                self.ssp_icr().modify(|_, w| {
                    if val {
                        w.roric().set_bit()
                    } else {
                        w.roric().clear_bit()
                    }
                });
            }

            fn rtic(&mut self, val: bool) {
                self.ssp_icr().modify(|_, w| {
                    if val {
                        w.rtic().set_bit()
                    } else {
                        w.rtic().clear_bit()
                    }
                });
            }

            fn rorim(&self) -> $crate::spi::InterruptMask {
                self.ssp_imsc().read().rorim().bit_is_set().into()
            }
            fn set_rorim(&mut self, val: $crate::spi::InterruptMask) {
                self.ssp_imsc().modify(|_, w| match val {
                    $crate::spi::InterruptMask::Masked => w.rorim().clear_bit(),
                    $crate::spi::InterruptMask::NotMasked => w.rorim().set_bit(),
                });
            }

            fn rtim(&self) -> $crate::spi::InterruptMask {
                self.ssp_imsc().read().rtim().bit_is_set().into()
            }
            fn set_rtim(&mut self, val: $crate::spi::InterruptMask) {
                self.ssp_imsc().modify(|_, w| match val {
                    $crate::spi::InterruptMask::Masked => w.rtim().clear_bit(),
                    $crate::spi::InterruptMask::NotMasked => w.rtim().set_bit(),
                });
            }

            fn rxim(&self) -> $crate::spi::InterruptMask {
                self.ssp_imsc().read().rxim().bit_is_set().into()
            }
            fn set_rxim(&mut self, val: $crate::spi::InterruptMask) {
                self.ssp_imsc().modify(|_, w| match val {
                    $crate::spi::InterruptMask::Masked => w.rxim().clear_bit(),
                    $crate::spi::InterruptMask::NotMasked => w.rxim().set_bit(),
                });
            }

            fn txim(&self) -> $crate::spi::InterruptMask {
                self.ssp_imsc().read().txim().bit_is_set().into()
            }
            fn set_txim(&mut self, val: $crate::spi::InterruptMask) {
                self.ssp_imsc().modify(|_, w| match val {
                    $crate::spi::InterruptMask::Masked => w.txim().clear_bit(),
                    $crate::spi::InterruptMask::NotMasked => w.txim().set_bit(),
                });
            }

            fn rormis(&self) -> bool {
                self.ssp_mis().read().rormis().bit_is_set()
            }
            fn rtmis(&self) -> bool {
                self.ssp_mis().read().rtmis().bit_is_set()
            }
            fn rxmis(&self) -> bool {
                self.ssp_mis().read().rxmis().bit_is_set()
            }
            fn txmis(&self) -> bool {
                self.ssp_mis().read().txmis().bit_is_set()
            }

            fn rorris(&self) -> bool {
                self.ssp_ris().read().rorris().bit_is_set()
            }
            fn rtris(&self) -> bool {
                self.ssp_ris().read().rtris().bit_is_set()
            }
            fn rxris(&self) -> bool {
                self.ssp_ris().read().rxris().bit_is_set()
            }
            fn txris(&self) -> bool {
                self.ssp_ris().read().txris().bit_is_set()
            }

            fn tfe(&self) -> bool {
                self.ssp_sr().read().tfe().bit_is_set()
            }
            fn tnf(&self) -> bool {
                self.ssp_sr().read().tnf().bit_is_set()
            }
            fn rne(&self) -> bool {
                self.ssp_sr().read().rne().bit_is_set()
            }
            fn rff(&self) -> bool {
                self.ssp_sr().read().rff().bit_is_set()
            }
            fn bsy(&self) -> bool {
                self.ssp_sr().read().bsy().bit_is_set()
            }
        }
    };
}

impl_spi_peripheral!(SPI0);
impl_spi_peripheral!(SPI1);
impl_spi_peripheral!(SPI2);
impl_spi_peripheral!(SPI3);
impl_spi_peripheral!(SPI4);
impl_spi_peripheral!(SPI5);
impl_spi_peripheral!(SPI6);
