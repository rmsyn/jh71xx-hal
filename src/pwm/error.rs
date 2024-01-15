use embedded_hal::pwm::{Error as PwmError, ErrorKind};

/// Convenience alias for the PWM module [`Result`](core::result::Result) type.
pub type Result<T> = core::result::Result<T, Error>;

/// Represents the PWM module error type.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Error {
    InvalidDutyCycle(u32),
    InvalidPeriod(u32),
    #[default]
    Other,
}

impl From<&Error> for ErrorKind {
    fn from(err: &Error) -> Self {
        match err {
            Error::InvalidDutyCycle(_cyc) => Self::Other,
            Error::InvalidPeriod(_per) => Self::Other,
            Error::Other => Self::Other,
        }
    }
}

impl From<Error> for ErrorKind {
    fn from(err: Error) -> Self {
        (&err).into()
    }
}

impl PwmError for Error {
    fn kind(&self) -> ErrorKind {
        self.into()
    }
}
