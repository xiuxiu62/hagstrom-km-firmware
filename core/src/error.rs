use std::sync::PoisonError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    SerialPort(#[from] serialport::Error),
    #[error("Lock poisoned: {0}")]
    Poison(String),
}

impl<T> From<PoisonError<T>> for Error {
    fn from(err: PoisonError<T>) -> Self {
        Self::Poison(err.to_string())
    }
}
