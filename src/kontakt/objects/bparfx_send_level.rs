use std::io::Cursor;

use crate::{
    kontakt::{error::KontaktError, structured_object::StructuredObject, Chunk},
    read_bytes::ReadBytesExt,
    Error,
};

const CHUNK_ID: u16 = 0x17;

/// Type:           Chunk<Data>
/// SerType:        0x17
/// Versions:       0x50, 0x51
/// Kontakt 7:      BParFXSendLevels
/// KontaktIO:
#[derive(Debug)]
pub struct BParFXSendLevels;

impl BParFXSendLevels {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        Ok(Self)
    }
}

impl std::convert::TryFrom<&Chunk> for BParFXSendLevels {
    type Error = Error;

    fn try_from(chunk: &Chunk) -> Result<Self, Self::Error> {
        if chunk.id != CHUNK_ID {
            return Err(KontaktError::IncorrectID {
                expected: CHUNK_ID,
                got: chunk.id,
            }
            .into());
        }
        let reader = Cursor::new(&chunk.data);
        Self::read(reader)
    }
}
