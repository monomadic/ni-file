use std::io::Cursor;

use crate::{
    kontakt::{Chunk, KontaktError},
    read_bytes::ReadBytesExt,
    Error,
};

#[derive(Debug)]
pub struct Bank;

impl Bank {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        Ok(Self)
    }
}

impl std::convert::TryFrom<&Chunk> for Bank {
    type Error = Error;

    fn try_from(chunk: &Chunk) -> Result<Self, Self::Error> {
        if chunk.id != 0x03 {
            return Err(KontaktError::IncorrectID {
                expected: 0x03,
                got: chunk.id,
            }
            .into());
        }
        let reader = Cursor::new(&chunk.data);
        Self::read(reader)
    }
}
