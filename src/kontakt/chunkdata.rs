use crate::{read_bytes::ReadBytesExt, Error};

pub struct ChunkData {
    pub id: u16,
    pub data: Vec<u8>,
}

impl ChunkData {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let id = reader.read_u16_le()?;
        let length = reader.read_u32_le()? as usize;
        let data = reader.read_bytes(length)?;

        // TODO: check size

        Ok(Self { id, data })
    }
}
