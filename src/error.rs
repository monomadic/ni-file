use crate::{nks::error::NKSError, read_bytes::ReadBytesError};

pub type Result<T> = std::result::Result<T, NIFileError>;
pub type Error = NIFileError;

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum NIFileError {
    #[error("Generic error: {0}")]
    Generic(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    NKSError(#[from] NKSError),

    #[error(transparent)]
    ReadBytesError(#[from] ReadBytesError),

    #[error("Decompression error")]
    DecompressionError,

    #[error("Incorrect Size Field: expected {expected}, got {got}")]
    IncorrectFrameSize { expected: u64, got: u64 },

    #[error("Encountered Item Terminator")]
    ItemTerminator,

    #[error("Static error: {0}")]
    Static(&'static str),
}
