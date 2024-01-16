use core::fmt;

use embedded_hal::spi::{Error as SpiError, ErrorKind};

use super::DataSize;

/// Convenience [`Result`](core::result::Result) alias for JH71xx SPI module.
pub type Result<T> = core::result::Result<T, Error>;

/// SPI error type for JH71xx SoCs.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Error {
    #[default]
    Overrun,
    ModeFault,
    FrameFormat,
    ChipSelectFault,
    Timeout,
    Other,
    DataSize(DataSize),
}

impl From<&Error> for ErrorKind {
    fn from(err: &Error) -> Self {
        match err {
            Error::Overrun => Self::Overrun,
            Error::ModeFault => Self::ModeFault,
            Error::FrameFormat => Self::FrameFormat,
            Error::ChipSelectFault => Self::ChipSelectFault,
            Error::Timeout => Self::Other,
            Error::Other => Self::Other,
            Error::DataSize(_ds) => Self::Other,
        }
    }
}

impl From<Error> for ErrorKind {
    fn from(err: Error) -> Self {
        (&err).into()
    }
}

impl SpiError for Error {
    fn kind(&self) -> ErrorKind {
        self.into()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Overrun => write!(f, "receive FIFO overrun"),
            Self::ModeFault => write!(f, "mode fault"),
            Self::FrameFormat => write!(f, "invalid frame format"),
            Self::ChipSelectFault => write!(f, "error asserting/deasserting chip select pin"),
            Self::Timeout => write!(f, "receive FIFO timeout"),
            Self::Other => write!(f, "other"),
            Self::DataSize(ds) => write!(f, "invalid data size: {ds}"),
        }
    }
}
