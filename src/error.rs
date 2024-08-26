use std::error::Error as StdError;
use std::{fmt::Display, io};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    E(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::E(error) => write!(f, "{}", error),
        }
    }
}

impl StdError for Error {}

impl From<io::Error> for Error {
    #[inline]
    fn from(value: io::Error) -> Self {
        Error::from(value.to_string())
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error::E(value)
    }
}

impl From<rbatis::Error> for Error {
    fn from(value: rbatis::Error) -> Self {
        Error::E(value.to_string())
    }
}
