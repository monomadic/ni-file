use std::io::Cursor;

use crate::{
    kontakt::{error::KontaktError, structured_object::StructuredObject, Chunk},
    Error,
};

const CHUNK_ID: u16 = 0x0D;

/// # InternalMod
///
/// ?
///
/// Type:           Chunk<StructuredObject>
/// SerType:        0x0D
/// Versions:       0x81
/// Kontakt 7:      BParInternalMod
/// KontaktIO:      ?
///
#[derive(Debug)]
pub struct InternalMod(pub StructuredObject);

impl InternalMod {}

impl std::convert::TryFrom<&Chunk> for InternalMod {
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

#[cfg(test)]
mod tests {
    use std::fs::File;

    use crate::Error;

    use super::*;

    #[test]
    fn test_bparam_array_v10() -> Result<(), Error> {
        let file = File::open("tests/data/Objects/Kontakt/0x0D-InternalMod/InternalMod-000.kon")?;
        let internalmod = InternalMod::try_from(&Chunk::read(file)?)?;
        assert_eq!(internalmod.0.version, 0x80);
        assert_eq!(internalmod.0.children.len(), 1);
        Ok(())
    }
}
