use thiserror::Error;

/// Error type for kvs.
#[derive(Error, Debug)]
pub enum KvsError {
    /// IO error.
    /// This error is returned when an IO error occurs.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    /// Serde error.
    /// This error is returned when a serde error occurs.
    #[error("serde error: {0}")]
    Serde(#[from] serde_json::Error),
    /// Key not found error.
    /// This error is returned when a key is not found.
    #[error("Key not found")]
    KeyNotFound,
    #[error("unknown data store error")]
    Unknown,
}

/// Result type for kvs.
pub type Result<T> = std::result::Result<T, KvsError>;
