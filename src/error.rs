use num_enum::{TryFromPrimitive, TryFromPrimitiveError};
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

impl<T: TryFromPrimitive> From<TryFromPrimitiveError<T>> for Error {
    fn from(value: TryFromPrimitiveError<T>) -> Self {
        Self::Error(format!("{}", value))
    }
}
