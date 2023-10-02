use std::io::Cursor;

use crate::{
    kontakt::{error::KontaktError, structured_object::StructuredObject, Chunk},
    read_bytes::ReadBytesExt,
    Error,
};

#[derive(Debug)]
pub struct BParFX(StructuredObject);

impl BParFX {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        Ok(Self(StructuredObject::read(&mut reader)?))
    }
}

impl std::convert::TryFrom<&Chunk> for BParFX {
    type Error = Error;

    fn try_from(chunk: &Chunk) -> Result<Self, Self::Error> {
        if chunk.id != 0x25 {
            return Err(KontaktError::IncorrectID {
                expected: 0x25,
                got: chunk.id,
            }
            .into());
        }
        let reader = Cursor::new(&chunk.data);
        Self::read(reader)
    }
}
