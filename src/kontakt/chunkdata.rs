use crate::{read_bytes::ReadBytesExt, Error};

pub struct ChunkData {
    id: u16,
    length: u32,
}

impl ChunkData {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        Ok(Self {
            id: reader.read_u16_le()?,
            length: reader.read_u32_le()?,
        })
    }
}
