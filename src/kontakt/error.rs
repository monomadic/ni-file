#[derive(thiserror::Error, Debug)]
pub enum KontaktError {
    #[error("Incorrect Chunk ID: expected 0x{expected:X}, got 0x{got:X}")]
    IncorrectID { expected: u16, got: u16 },

    #[error("Missing Expected Chunk: 0x{0}")]
    MissingChunk(u16),
}
