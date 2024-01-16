use core::fmt;

use embedded_hal::digital::Error as GpioError;
pub use embedded_hal::digital::ErrorKind;

/// Convenience [`Result`](core::result::Result) alias for JH71xx GPIO module.
pub type Result<T> = core::result::Result<T, Error>;

/// GPIO errors.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Error {
    InvalidPad(u32),
}

impl GpioError for Error {
    fn kind(&self) -> ErrorKind {
        ErrorKind::Other
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPad(err) => write!(f, "invalid pad number: {err}"),
        }
    }
}
