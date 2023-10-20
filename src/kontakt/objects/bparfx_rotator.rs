use std::io::Cursor;

use crate::{
    kontakt::{error::KontaktError, Chunk},
    read_bytes::ReadBytesExt,
    Error,
};

const CHUNK_ID: u16 = 0x22;

/// Type:           Chunk<Data>
/// SerType:        0x22
/// Versions:       ?
/// Kontakt 7:      BParFXRotator
/// KontaktIO:
#[derive(Debug)]
pub struct BParFXRotator;

impl BParFXRotator {
    pub fn read<R: ReadBytesExt>(mut _reader: R) -> Result<Self, Error> {
        Ok(Self)
    }
}

impl std::convert::TryFrom<&Chunk> for BParFXRotator {
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
