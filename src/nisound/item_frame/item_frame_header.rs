use crate::nisound::item_frame::item_id::ItemID;
use crate::nisound::Domain;
use crate::prelude::*;
use crate::read_bytes::ReadBytesExt;

#[derive(Debug, Clone)]
pub struct ItemFrameHeader {
    size: u64,
    pub domain: Domain,
    pub item_id: ItemID,
    version: u32,
}

impl ItemFrameHeader {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {
        log::debug!("ItemFrameHeader::read");
        Ok(Self {
            size: reader.read_u64_le()?,
            domain: reader.read_u32_le()?.into(),
            item_id: ItemID::from(reader.read_u32_le()?),
            version: reader.read_u32_le()?,
        })
    }
}
