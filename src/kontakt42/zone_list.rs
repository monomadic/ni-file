use crate::{read_bytes::ReadBytesExt, Error};

pub struct ZoneList;

impl ZoneList {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        // length
        assert!(reader.read_u32_le()? > 0);

        println!("{}", reader.read_u32_le()?);
        Ok(Self {})
    }
}
