use crate::{prelude::*, read_bytes::ReadBytesExt, repository::item_frame::ItemFrame, ItemID};

#[derive(Debug, Clone)]
pub struct PresetChunkItem(Vec<u8>);

impl std::convert::TryFrom<ItemFrame> for PresetChunkItem {
    type Error = NIFileError;

    fn try_from(frame: ItemFrame) -> std::result::Result<Self, Self::Error> {
        log::debug!("PresetChunkItem::try_from");
        debug_assert_eq!(frame.header.item_id, ItemID::PresetChunkItem);
        PresetChunkItem::read(frame.data.as_slice())
    }
}

impl PresetChunkItem {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {
        log::debug!("PresetChunkItem::read");

        // version == 1
        assert_eq!(reader.read_u32_le()?, 1);

        // auth checksum
        let auth_checksum = reader.read_u32_le()?;
        log::debug!("auth_checksum: {}", auth_checksum);

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
