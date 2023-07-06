use crate::{kontakt42::zone::ZoneData, read_bytes::ReadBytesExt, Error};

pub struct ZoneList {
    zones: Vec<ZoneData>,
}

impl ZoneList {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let array_length = reader.read_u32_le()?;
        println!("array_length {}", array_length);

        let num_children = reader.read_u32_le()?;
        println!("num_children {}", num_children);

        let is_chunked = reader.read_bool()?;
        println!("is_chunked {:?}", is_chunked);

        let mut zones = Vec::new();
        if is_chunked {
            let version = reader.read_u16_le()?;
            for _i in 0..array_length {
                zones.push(ZoneData::from_version(&mut reader, version)?);
            }
        }

        Ok(Self { zones })
    }
}

#[test]
fn test_zone_list() -> Result<(), Error> {
    let file = include_bytes!("tests/zone_list/4.2.2.4504/000");
    assert!(ZoneList::read(file.as_slice()).is_ok());
    Ok(())
}
