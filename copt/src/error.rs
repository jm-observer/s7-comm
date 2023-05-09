use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    IoErr(#[from] io::Error),

    // #[error(transparent)]
    // AnyhowErr(#[from] anyhow::Error),
    #[error("Error: {0}")]
    Error(String),
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait ToCoptError {
    fn to_err(self) -> Error;
}

impl<T: ToCoptError> From<T> for Error {
    fn from(value: T) -> Self {
        value.to_err()
    }
}
