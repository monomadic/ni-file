use crate::{read_bytes::ReadBytesExt, Error};

pub struct ChunkData {
    pub id: u16,
    pub data: Vec<u8>,
}

impl ChunkData {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let id = reader.read_u16_le()?;
        let length = reader.read_u32_le()?;
        let data = reader.read_bytes(length as usize)?;

        Ok(Self { id, data })
    }
}
