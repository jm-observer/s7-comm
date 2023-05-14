use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    IoErr(#[from] io::Error),

    #[error(transparent)]
    TpktErr(#[from] tpkt::Error),

    #[error("Error: {0}")]
    Error(String),
}

pub type Result<T> = std::result::Result<T, Error>;

// impl<T: TryFromPrimitive> From<TryFromPrimitiveError<T>> for Error {
//     fn from(value: TryFromPrimitiveError<T>) -> Self {
//         Self::Error(format!("{}", value))
//     }
// }
//
// impl ToCoptError for Error {
//     fn to_err(self) -> copt::error::Error {
//         copt::error::Error::Error(self.to_string())
//     }
// }
