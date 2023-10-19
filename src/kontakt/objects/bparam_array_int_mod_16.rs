use std::io::Cursor;

use crate::{
    kontakt::{error::KontaktError, structured_object::StructuredObject, Chunk},
    read_bytes::ReadBytesExt,
    Error,
};

const CHUNK_ID: u16 = 0x3B;

/// BParameterArraySerBParInternalMod16
///
/// An array of 16 InternalMod objects.
///
/// Type:           Chunk<StructuredObject>
/// SerType:        0x3B
/// Versions:       0x10, 0x11, 0x12
/// Kontakt 7:      BParameterArraySerBParInternalMod16
/// KontaktIO:      BParamArray<16>
#[derive(Debug)]
pub struct InternalModArray16(pub StructuredObject);

impl InternalModArray16 {
    pub fn children(&self) -> Result<Vec<Chunk>, Error> {
        let mut reader = Cursor::new(&self.0.public_data);
        let mut items = Vec::new();

        match self.0.version {
            0x11 => unimplemented!(),
            0x10 | 0x12 => {
                for _ in 0..16 {
                    if reader.read_bool()? {
                        // if V11:
                        // reader.read_u32_le()?;
                        items.push(Chunk::read(&mut reader)?);
                    }
                }
            }
            _ => panic!(
                "Unsupported InternalModArray16 version: 0x{:X}",
                self.0.version
            ),
        }

        Ok(items)
    }
}

impl std::convert::TryFrom<&Chunk> for InternalModArray16 {
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
        let file = File::open(
            "tests/data/Objects/Kontakt/0x3B-InternalModArray16/InternalModArray16-000.kon",
        )?;
        let mod_arr = InternalModArray16::try_from(&Chunk::read(file)?)?;
        assert_eq!(mod_arr.0.version, 0x10);
        assert_eq!(mod_arr.0.children.len(), 0);
        assert_eq!(mod_arr.children()?.len(), 3);
        Ok(())
    }
}
