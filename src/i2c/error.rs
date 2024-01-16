use core::convert::Infallible;

use embedded_hal::i2c::{Error as I2cError, ErrorKind, NoAcknowledgeSource};

/// Convenience [`Result`](core::result::Result) alias for JH71xx I2C module.
pub type Result<T> = core::result::Result<T, Error>;

/// I2C Error types
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Error {
    /// Bus error
    Bus,
    /// The arbitration was lost, e.g. electrical problems with the clock signal.
    ArbitrationLoss,
    /// No ACK received
    NoAcknowledge(NoAcknowledgeSource),
    /// The peripheral receive buffer was overrun.
    Overrun,
    /// A different error occurred. The original error may contain more information.
    Other,
}

impl From<&Error> for ErrorKind {
    fn from(val: &Error) -> Self {
        match val {
            Error::Bus => Self::Bus,
            Error::ArbitrationLoss => Self::ArbitrationLoss,
            Error::NoAcknowledge(src) => Self::NoAcknowledge(*src),
            Error::Overrun => Self::Overrun,
            Error::Other => Self::Other,
        }
    }
}

impl From<Error> for ErrorKind {
    fn from(val: Error) -> Self {
        (&val).into()
    }
}

impl I2cError for Error {
    fn kind(&self) -> ErrorKind {
        self.into()
    }
}

impl From<Infallible> for Error {
    fn from(_err: Infallible) -> Self {
        Self::Other
    }
}
