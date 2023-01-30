//! Main Crate Error
pub use thiserror::Error;
pub type Result<T> = std::result::Result<T, NIFileError>;

#[derive(Error, Debug)]
pub enum NIFileError {
    /// For starter, to remove as code matures.
    #[error("Generic error: {0}")]
    Generic(String),
    /// For starter, to remove as code matures.
    #[error("Static error: {0}")]
    Static(&'static str),

    #[error("Incorrect Size Field: expected {expected}, got {got}")]
    IncorrectFrameSize { expected: u64, got: u64 },

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    Binread(#[from] binread::Error),

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}
