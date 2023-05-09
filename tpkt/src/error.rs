use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    IoErr(#[from] io::Error),

    #[error("Error: {0}")]
    Error(String),
}

pub trait ToTpktError {
    fn to_err(self) -> Error;
}

impl<T: ToTpktError> From<T> for Error {
    fn from(value: T) -> Self {
        value.to_err()
    }
}
