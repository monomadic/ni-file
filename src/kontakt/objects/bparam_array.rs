use crate::{kontakt::chunk::Chunk, read_bytes::ReadBytesExt, Error};

// id 0x3a
// known versions: 0x10, 0x11, 0x12
#[doc = include_str!("../../../doc/presets/Kontakt/BParamArray.md")]
#[derive(Debug)]
pub struct BParamArray(Vec<Chunk>);

impl BParamArray {
    pub fn read<R: ReadBytesExt>(mut reader: R, num_items: u32) -> Result<Self, Error> {
        let is_structured_data = reader.read_bool()?;
        assert!(!is_structured_data); // always false?

        let version = reader.read_u16_le()?;

        let mut items = Vec::new();
        match version {
            0x11 => {
                panic!("Unsupported BParamArray: v11");
            }
            _ => {
                for _ in 0..num_items {
                    let has_item = reader.read_bool()?;
                    if has_item {
                        let chunk = Chunk::read(&mut reader)?;
                        items.push(chunk);
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
        let file = File::open("tests/data/Objects/KontaktV42/BParameterArray/BParameterArray-001")?;
        let arr = BParamArray::read(file, 8)?;
        // dbg!(arr);
        Ok(())
    }

    #[test]
    fn test_bparam_array_v12() -> Result<(), Error> {
        let file = File::open("tests/data/Objects/KontaktV42/BParameterArray/BParameterArray-000")?;
        let arr = BParamArray::read(file, 8)?;
        // dbg!(arr);
        Ok(())
    }
}
