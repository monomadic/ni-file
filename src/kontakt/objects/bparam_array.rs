use crate::{kontakt::chunk::Chunk, read_bytes::ReadBytesExt, Error};

use super::BParFX;

/// Type:           StructuredObject
/// SerType:        0x3A
/// Versions:       0x10, 0x11, 0x12
/// Kontakt 7:      BParameterArraySerBParFX8
/// KontaktIO:      BParamArray<8>
#[doc = include_str!("../../../doc/presets/Kontakt/BParamArray.md")]
#[derive(Debug)]
pub struct BParamArrayBParFX8(Vec<Option<BParFX>>);

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
                        let chunk = Chunk::read(&mut reader)?;
                        items.push(Some((&chunk).try_into()?));
                    } else {
                        items.push(None);
                    }
                }
            }
        }

        Ok(Self(items))
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

        assert_eq!(arr.0.len(), 8);
        Ok(())
    }

    #[test]
    fn test_bparam_array_v12() -> Result<(), Error> {
        let file = File::open("tests/data/Objects/Kontakt/BParameterArray/BParameterArray-000")?;
        let arr = BParamArrayBParFX8::read(file, 8)?;

        assert_eq!(arr.0.len(), 8);
        Ok(())
    }
}
