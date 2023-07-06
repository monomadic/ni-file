use crate::{read_bytes::ReadBytesExt, Error};

// id 0x3a
#[derive(Debug)]
pub struct BParamArray;

impl BParamArray {
    pub fn read<R: ReadBytesExt>(mut reader: R, items: u32) -> Result<Self, Error> {
        println!("BParamArray::read(items={})", items);
        println!("{:?}", reader.read_bool()?);
        println!("u16: {}", reader.read_u16_le()?);

        for _ in 0..items {
            let do_read = reader.read_bool()?;
            println!("bool: {}", do_read);

            if do_read {
                println!("id: 0x{:x}", reader.read_u16_le()?);
                let len = reader.read_u32_le()?;
                println!("len: {}", len);
                let inner = reader.read_bytes(len as usize)?;
                let mut reader = inner.as_slice();

                assert_eq!(reader.read_u8()?, 1);
                assert_eq!(reader.read_u16_le()?, 0x50); // id
                let len = reader.read_u32_le()?;
                let inner = reader.read_bytes(len as usize)?;

                // let len = reader.read_u32_le()?;
                // let inner = reader.read_bytes(len as usize)?;
            }
        }

        Ok(Self)
    }
}

#[test]
fn test_zone_list() -> Result<(), Error> {
    let file = include_bytes!("tests/param_array/4.2.2.4504/000");
    assert!(BParamArray::read(file.as_slice(), 8).is_ok());
    Ok(())
}
