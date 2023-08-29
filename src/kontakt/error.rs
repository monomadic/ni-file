#[derive(thiserror::Error, Debug)]
pub enum KontaktError {
    #[error("Incorrect Chunk ID: expected {expected}, got {got}")]
    IncorrectID { expected: u16, got: u16 },
}
