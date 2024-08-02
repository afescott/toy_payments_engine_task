use thiserror::Error;

use crate::models::Record;

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

/// A Dexscreener error.
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    SendError(#[from] std::sync::mpsc::SendError<Record>),

    #[error(transparent)]
    CsvError(#[from] csv::Error),
}
