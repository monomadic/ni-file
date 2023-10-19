use crate::{
    kontakt::{error::KontaktError, structured_object::StructuredObject, Chunk},
    Error,
};

const CHUNK_ID: u16 = 0x25;

/// # BParFX
///
/// An effect slot. The effect object is the first child.
/// Contains private data but no public data.
///
/// - Type:           Chunk<StructuredObject>
/// - SerType:        0x25
/// - Versions:       0x50
/// - Kontakt 7:      BParameterArraySerBParFX8
/// - KontaktIO:      BParamArray<8>
///
#[derive(Debug)]
pub struct BParFX(pub StructuredObject);

impl BParFX {
    pub fn version(&self) -> u16 {
        self.0.version
    }

    pub fn effect(&self) -> Option<&Chunk> {
        self.0.children.get(0)
    }
}

impl std::convert::TryFrom<&Chunk> for BParFX {
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
