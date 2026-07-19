//! Error types for Valence routing and backends.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Privacy policy violation: {0}")]
    Privacy(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Pending deletion: {0}")]
    PendingDeletion(String),

    #[error("Identity error: {0}")]
    Identity(String),
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Serialization(e.to_string())
    }
}

impl Error {
    /// True when the database engine reported MVCC / transaction contention that may succeed on retry.
    pub fn is_retryable_transaction_contention(&self) -> bool {
        match self {
            Error::Database(msg) => {
                let s = msg.to_lowercase();
                s.contains("read or write conflict")
                    || s.contains("can be retried")
                    || (s.contains("failed transaction") && s.contains("conflict"))
            }
            _ => false,
        }
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Validation(s.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
