use crate::{read_bytes::ReadBytesExt, Error};

pub struct FileNameListPreK51;

impl FileNameListPreK51 {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        println!("FileNameListPreK51::read");

        let length = reader.read_u32_le()?;
        println!("length {}", length);

        let _data = reader.read_bytes(length as usize)?;

        Ok(Self {})
    }
}
