/// Convenience [`Result`](core::result::Result) alias for JH71xx UART module.
pub type Result<T> = core::result::Result<T, Error>;

/// Error type for the UART peripheral
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    ReadTimeout,
    WriteTimeout,
    ReadOverrun,
    WriteOverrun,
    WouldBlock,
}

impl From<&Error> for io::ErrorKind {
    fn from(err: &Error) -> Self {
        match err {
            Error::ReadTimeout | Error::WriteTimeout => Self::TimedOut,
            Error::ReadOverrun => Self::InvalidInput,
            Error::WriteOverrun => Self::InvalidData,
            Error::WouldBlock => Self::Other,
        }
    }
}

impl From<&Error> for embedded_hal_nb::serial::ErrorKind {
    fn from(err: &Error) -> Self {
        match err {
            Error::ReadTimeout | Error::WriteTimeout => Self::Other,
            Error::ReadOverrun => Self::Overrun,
            Error::WriteOverrun => Self::Overrun,
            Error::WouldBlock => Self::Other,
        }
    }
}

impl From<Error> for io::ErrorKind {
    fn from(err: Error) -> Self {
        (&err).into()
    }
}

impl From<nb::Error<Error>> for Error {
    fn from(err: nb::Error<Error>) -> Self {
        match err {
            nb::Error::WouldBlock => Self::WouldBlock,
            nb::Error::Other(err) => err,
        }
    }
}

impl io::Error for Error {
    fn kind(&self) -> io::ErrorKind {
        self.into()
    }
}

impl embedded_hal_nb::serial::Error for Error {
    fn kind(&self) -> embedded_hal_nb::serial::ErrorKind {
        self.into()
    }
}
