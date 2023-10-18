use crate::{
    kontakt::{error::KontaktError, Chunk},
    Error,
};

pub const KONTAKT_PRIVATE_RAW_OBJECT_ID: u16 = 0x35;

/// Type:           Chunk
/// SerType:        0x35
/// Kontakt 7:      ?
/// KontaktIO:      PrivateRawObject
#[derive(Debug)]
pub struct PrivateRawObject(Vec<u8>);

impl PrivateRawObject {
    pub fn data(&self) -> &Vec<u8> {
        &self.0
    }
}

impl std::convert::TryFrom<&Chunk> for PrivateRawObject {
    type Error = Error;

    fn try_from(chunk: &Chunk) -> Result<Self, Self::Error> {
        if chunk.id != KONTAKT_PRIVATE_RAW_OBJECT_ID {
            return Err(KontaktError::IncorrectID {
                expected: KONTAKT_PRIVATE_RAW_OBJECT_ID,
                got: chunk.id,
            }
            .into());
        }
        Ok(Self(chunk.data.clone()))
    }
}
