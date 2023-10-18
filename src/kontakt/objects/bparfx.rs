use crate::{
    kontakt::{error::KontaktError, structured_object::StructuredObject, Chunk},
    Error,
};

const CHUNK_ID: u16 = 0x25;

/// # BParFX
///
/// A wrapper for an effect parameter. The effect is the first child.
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
