#[derive(thiserror::Error, Debug)]
pub enum NKSError {
    #[error("Invalid magic number. Expected: 0x7fa89012, 0x5EE56EB3, got: 0x{0:x}")]
    InvalidMagicNumber(u32),

    #[error(transparent)]
    IO(#[from] std::io::Error),
}
