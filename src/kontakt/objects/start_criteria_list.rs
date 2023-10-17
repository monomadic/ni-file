use std::io::Cursor;

use crate::{
    kontakt::{Chunk, KontaktError},
    read_bytes::ReadBytesExt,
    Error,
};

/// Type:           Chunk
/// SerType:        ?
/// Kontakt 7:      ?
/// KontaktIO:      StartCritList
#[derive(Debug)]
pub struct StartCriteriaList {
    pub group_starts: u8,
}

impl StartCriteriaList {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        // Always, Start On Key, Start On Controller,
        // Cycle Round Robin, Cycle Random, Slice Trigger
        Ok(Self {
            group_starts: reader.read_u8()?,
        })
    }
}

impl std::convert::TryFrom<&Chunk> for StartCriteriaList {
    type Error = Error;

    fn try_from(chunk: &Chunk) -> Result<Self, Self::Error> {
        if chunk.id != 0x38 {
            return Err(KontaktError::IncorrectID {
                expected: 0x38,
                got: chunk.id,
            }
            .into());
        }
        let reader = Cursor::new(&chunk.data);
        Self::read(reader)
    }
}
