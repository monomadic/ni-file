//! Main Crate Error
pub use thiserror::Error;

pub type Result<T> = std::result::Result<T, NIFileError>;

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum NIFileError {
    #[error("Generic error: {0}")]
    Generic(String),

    #[error("Static error: {0}")]
    Static(&'static str),

    // #[error("Unexpected Frame: expected {expected:?}, got {got:?}")]
    // UnexpectedFrame { expected: ItemFrame, got: ItemFrame },
    #[error("Incorrect Size Field: expected {expected}, got {got}")]
    IncorrectFrameSize { expected: u64, got: u64 },

    #[error("Encountered Item Terminator")]
    ItemTerminator,

    #[error(transparent)]
    IO(#[from] std::io::Error),
}
