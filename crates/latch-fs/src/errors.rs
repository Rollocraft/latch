//! Error type for the filesystem adapter.

use latch_transaction::TransactionError;

#[derive(Debug)]
pub enum FileError {
    UnknownAction(String),
    UnknownBudget(String),
    /// A write and a rename carry exactly one argument, a delete none.
    MalformedAction,
    Transaction(TransactionError),
}

impl std::fmt::Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownAction(name) => write!(f, "unsupported file action {name:?}"),
            Self::UnknownBudget(name) => write!(f, "cannot price budget {name:?}"),
            Self::MalformedAction => write!(f, "file action has the wrong arguments"),
            Self::Transaction(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for FileError {}

impl From<TransactionError> for FileError {
    fn from(error: TransactionError) -> Self {
        Self::Transaction(error)
    }
}
