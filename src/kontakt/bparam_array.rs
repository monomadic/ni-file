use crate::{kontakt::chunkdata::ChunkData, read_bytes::ReadBytesExt, Error};

// id 0x3a
#[derive(Debug)]
pub struct BParamArray;

impl BParamArray {
    pub fn read<R: ReadBytesExt>(mut reader: R, items: u32) -> Result<Self, Error> {
        println!("BParamArray::read(items={})", items);
        println!("? {:?}", reader.read_bool()?);

        let version = reader.read_u16_le()?;
        println!("version: 0x{:x}", version);

        match version {
            0x11 => {
                panic!("version 0x17");
            }
            _ => {
                for i in 0..items {
                    let do_read = reader.read_bool()?;
                    println!("item[{}] {}", i, do_read);

                    if do_read {
                        // StructuredObject::read(&mut reader)?;
                        let ChunkData { id, data } = ChunkData::read(&mut reader)?;
                        let mut reader = data.as_slice();

                        println!("array item: 0x{:x}", id);

                        assert_eq!(reader.read_u8()?, 1);

                        assert_eq!(reader.read_u16_le()?, 0x50); // version?
                        let len = reader.read_u32_le()?;
                        let _inner = reader.read_bytes(len as usize)?;
                        let len = reader.read_u32_le()?;
                        let _inner = reader.read_bytes(len as usize)?;
                    }
                }
            }
        }

        Ok(Self)
    }
}

#[test]
fn test_bparam_array() -> Result<(), Error> {
    let file = include_bytes!("tests/param_array/4.2.2.4504/000");
    assert!(BParamArray::read(file.as_slice(), 8).is_ok());
    Ok(())
}
