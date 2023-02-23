use crate::prelude::*;
use crate::read_bytes::ReadBytesExt;

pub struct ItemFrameHeader {
    size: u64,
    pub domain_id: u32,
    pub item_id: u32,
    version: u32,
}

impl ItemFrameHeader {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {
        // TODO: validation
        Ok(Self {
            size: reader.read_u64_le()?,
            domain_id: reader.read_u32_le()?,
            item_id: reader.read_u32_le()?,
            version: reader.read_u32_le()?,
        })
    }
}
