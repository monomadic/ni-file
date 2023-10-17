use std::io::Cursor;

use crate::{
    kontakt::{Chunk, KontaktError},
    read_bytes::ReadBytesExt,
    Error,
};

/// Type:           Chunk
/// SerType:        0x39
/// Kontakt 7:      ?
/// KontaktIO:      LoopArray
#[derive(Debug)]
pub struct LoopArray;

impl LoopArray {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let num_items = reader.read_u8()?;

        for _ in 0..num_items {}

        Ok(Self)
    }
}

impl std::convert::TryFrom<&Chunk> for LoopArray {
    type Error = Error;

    fn try_from(chunk: &Chunk) -> Result<Self, Self::Error> {
        if chunk.id != 0x32 {
            return Err(KontaktError::IncorrectID {
                expected: 0x32,
                got: chunk.id,
            }
            .into());
        }
        let reader = Cursor::new(&chunk.data);
        Self::read(reader)
    }
}
