use crate::read_bytes::ReadBytesError;

#[derive(thiserror::Error, Debug)]
pub enum NKSError {
    #[error("Invalid magic number. Expected: 0x7FA89012, 0x5EE56EB3, got: 0x{0:x}")]
    InvalidMagicNumber(u32),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    ReadBytesError(#[from] ReadBytesError),

    #[error("Decompression error: {0}")]
    Decompression(String),
}
