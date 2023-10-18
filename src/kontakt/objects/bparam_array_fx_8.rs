use std::io::Cursor;

use crate::{
    kontakt::{chunk::Chunk, KontaktError},
    read_bytes::ReadBytesExt,
    Error,
};

use super::BParFX;

/// Type:           StructuredObject
/// SerType:        0x3A
/// Versions:       0x10, 0x11, 0x12
/// Kontakt 7:      BParameterArraySerBParFX8
/// KontaktIO:      BParamArray<8>
#[doc = include_str!("../../../doc/presets/Kontakt/BParamArray.md")]
#[derive(Debug)]
pub struct BParamArrayBParFX8 {
    pub version: u16,
    pub items: Vec<Option<Chunk>>,
}

impl BParamArrayBParFX8 {
    pub fn read<R: ReadBytesExt>(mut reader: R, num_items: u32) -> Result<Self, Error> {
        let is_structured_data = reader.read_bool()?;
        let version = reader.read_u16_le()?;
        let mut items = Vec::new();

        assert!(!is_structured_data); // always false?

        match version {
            0x11 => {
                panic!("Unsupported BParamArrayBParFX8: v11");
            }
            _ => {
                for _ in 0..num_items {
                    let has_item = reader.read_bool()?;
                    if has_item {
                        items.push(Some(Chunk::read(&mut reader)?));
                    } else {
                        items.push(None);
                    }
                }
            }
        }

        Ok(Self { version, items })
    }

    pub fn fx_items(&self) -> Result<Vec<BParFX>, Error> {
        Ok(self
            .items
            .iter()
            .filter_map(|c| c.as_ref())
            .filter_map(|c| BParFX::try_from(c).ok())
            .collect())
    }

    pub fn len(&self) -> usize {
        self.items
            .iter()
            .filter(|p| p.is_some())
            .collect::<Vec<&Option<_>>>()
            .len()
    }
}

impl std::convert::TryFrom<&Chunk> for BParamArrayBParFX8 {
    type Error = Error;

    fn try_from(chunk: &Chunk) -> Result<Self, Self::Error> {
        if chunk.id != 0x3A {
            return Err(KontaktError::IncorrectID {
                expected: 0x3A,
                got: chunk.id,
            }
            .into());
        }
        let reader = Cursor::new(&chunk.data);
        Self::read(reader, 8)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use crate::Error;

    use super::*;

    #[test]
    fn test_bparam_array_v10() -> Result<(), Error> {
        let file = File::open("tests/data/Objects/Kontakt/BParameterArray/BParameterArray-001")?;
        let arr = BParamArrayBParFX8::read(file, 8)?;

        assert_eq!(arr.items.len(), 8);
        Ok(())
    }

    #[test]
    fn test_bparam_array_v12() -> Result<(), Error> {
        let file = File::open("tests/data/Objects/Kontakt/BParameterArray/BParameterArray-000")?;
        let arr = BParamArrayBParFX8::read(file, 8)?;

        assert_eq!(arr.items.len(), 8);
        Ok(())
    }
}
