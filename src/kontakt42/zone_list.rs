use crate::{read_bytes::ReadBytesExt, Error};

pub struct ZoneList;

impl ZoneList {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let array_length = reader.read_u32_le()?;
        println!("array_length {}", array_length);

        Ok(Self {})
    }
}
