use sea_orm::DbErr;
use thiserror::Error;

/// Core error enum that encompasses all error types
#[derive(Error, Debug)]
pub enum Error {
    /// sea_orm database error
    #[error(transparent)]
    Database(#[from] DbErr),

    /// Custom error with a message
    #[error("{0}")]
    Custom(String),

    /// Transaction-related error
    #[error("transaction error: {0}")]
    Transaction(String),
}

pub type Result<T> = std::result::Result<T, Error>;
