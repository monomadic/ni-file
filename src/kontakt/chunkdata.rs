use crate::{read_bytes::ReadBytesExt, Error, NIFileError};

pub struct ChunkData {
    pub id: u16,
    pub data: Vec<u8>,
}

impl ChunkData {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let id = reader.read_u16_le()?;
        let length = reader.read_u32_le()?;
        let data = reader.read_bytes(length as usize).map_err(|_| {
            NIFileError::Generic(format!(
                "Failed to read ChunkData: id=0x{id:x}, length={length}"
            ))
        })?;

        Ok(Self { id, data })
    }
}
