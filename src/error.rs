use std::{
    fmt::{Display, Formatter},
    string::FromUtf8Error,
};

#[derive(Debug)]
pub enum Error {
    UTF8Error,
    Bincode(bincode::Error),
}

impl From<bincode::Error> for Error {
    #[inline]
    fn from(enc: bincode::Error) -> Self {
        Self::Bincode(enc)
    }
}

impl From<FromUtf8Error> for Error {
    #[inline]
    fn from(_: FromUtf8Error) -> Self {
        Self::UTF8Error
    }
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
