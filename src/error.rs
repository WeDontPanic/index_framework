use std::{fmt::Display, string::FromUtf8Error};

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    UTF8Error,
    Decode,
    InvalidIndex,
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
