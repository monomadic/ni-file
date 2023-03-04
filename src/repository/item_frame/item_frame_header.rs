use crate::prelude::*;
use crate::read_bytes::ReadBytesExt;
use crate::repository::item_frame::domain_id::DomainID;
use crate::repository::item_frame::item_id::ItemID;

#[derive(Debug, Clone)]
pub struct ItemFrameHeader {
    size: u64,
    pub domain_id: DomainID,
    pub item_id: ItemID,
    version: u32,
}

impl ItemFrameHeader {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {
        log::debug!("ItemFrameHeader::read");
        Ok(Self {
            size: reader.read_u64_le()?,
            domain_id: DomainID(reader.read_u32_le()?),
            item_id: ItemID::from(reader.read_u32_le()?),
            version: reader.read_u32_le()?,
        })
    }
}
