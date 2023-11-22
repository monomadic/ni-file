use std::io::Cursor;

use crate::{
    nis::{ItemData, ItemType},
    read_bytes::ReadBytesExt,
    NIFileError,
};

/// Typically contains the binary chunk for the inner NISound document.
#[derive(Debug, Clone)]
pub struct PresetChunkItemProperties(pub Vec<u8>);

impl std::convert::TryFrom<&ItemData> for PresetChunkItemProperties {
    type Error = NIFileError;

    fn try_from(frame: &ItemData) -> std::result::Result<Self, Self::Error> {
        debug_assert_eq!(frame.header.item_type(), ItemType::PresetChunkItem);
        PresetChunkItemProperties::read(Cursor::new(&frame.data))
    }
}

impl PresetChunkItemProperties {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, NIFileError> {
        // version == 1
        assert_eq!(reader.read_u32_le()?, 1);

        // auth checksum
        let _auth_checksum = reader.read_u32_le()?;

        // BinaryChunk::read
        assert_eq!(reader.read_u32_le()?, 1);
        let size = reader.read_u64_le()? as usize;
        let chunk = reader.read_bytes(size)?;

        Ok(Self(chunk))
    }

    pub fn chunk(&self) -> &Vec<u8> {
        &self.0
    }
}
