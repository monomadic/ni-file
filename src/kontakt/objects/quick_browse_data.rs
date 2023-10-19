use std::io::Cursor;

use crate::{
    kontakt::{error::KontaktError, structured_object::StructuredObject, Chunk},
    read_bytes::ReadBytesExt,
    Error,
};

const CHUNK_ID: u16 = 0x4E;

/// QuickBrowseData
///
/// Who knows, some crap some NI middle manager thought was
/// a good idea at the time.
///
/// Type:           Chunk<StructuredObject>
/// SerType:        0x4E
/// Versions:       0x1
/// Kontakt 7:      QuickBrowseData
/// KontaktIO:
#[derive(Debug)]
pub struct QuickBrowseData(pub StructuredObject);

#[derive(Debug)]
pub struct QuickBrowseDataParams {
    pub unknown: i32,
}

impl QuickBrowseData {
    pub fn params(&self) -> Result<QuickBrowseDataParams, Error> {
        let mut reader = Cursor::new(&self.0.public_data);
        Ok(QuickBrowseDataParams {
            unknown: reader.read_i32_le()?,
        })
    }
}

impl std::convert::TryFrom<&Chunk> for QuickBrowseData {
    type Error = Error;

    fn try_from(chunk: &Chunk) -> Result<Self, Self::Error> {
        if chunk.id != CHUNK_ID {
            return Err(KontaktError::IncorrectID {
                expected: CHUNK_ID,
                got: chunk.id,
            }
            .into());
        }
        Ok(Self(chunk.try_into()?))
    }
}
