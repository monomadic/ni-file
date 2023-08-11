use crate::nisound::item_frame::item_id::ItemID;
use crate::nisound::Domain;
use crate::prelude::*;
use crate::read_bytes::ReadBytesExt;

/// 20 bytes
#[derive(Debug, Clone)]
pub struct ItemFrameHeader {
    pub length: u64,
    pub domain: Domain,
    pub item_id: ItemID,
    pub version: u32,
}

impl ItemFrameHeader {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {
        Ok(Self {
            length: reader.read_u64_le()?,
            domain: reader.read_u32_le()?.into(),
            item_id: ItemID::from(reader.read_u32_le()?),
            version: reader.read_u32_le()?,
        })
    }
}
