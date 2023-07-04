use crate::{
    kontakt42::structured_object::StructuredObjectReader, read_bytes::ReadBytesExt, Error,
};

pub struct ZoneList;

impl ZoneList {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let array_length = reader.read_u32_le()?;
        println!("array_length {}", array_length);

        let num_children = reader.read_u32_le()?;
        println!("num_children {}", num_children);

        let obj = StructuredObjectReader {
            id: 0x2c,
            length: 0,
        };
        obj.do_read(&mut reader)?;

        // StructuredObject::doRead(0x2c);
        // StructuredObject::read(&mut reader)?;

        Ok(Self {})
    }
}

#[test]
fn test_zone_list() -> Result<(), Error> {
    let file = include_bytes!("tests/zone_list/4.2.2.4504/000");
    assert!(ZoneList::read(file.as_slice()).is_ok());
    Ok(())
}
