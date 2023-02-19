use crate::read_bytes::ReadBytesExt;

use super::ItemError;

#[derive(Debug, Clone)]
pub struct ItemHeader {
    pub item_id: u32,      // (+0x8, uint)
    pub domain_id: u32,    // (+0xC, uint, 'hsin')
    pub header_flags: u32, // (0x10, uint)
    pub uuid: Vec<u8>,     // (0x14, int32_t)
}

impl ItemHeader {
    pub fn read<R>(mut reader: R) -> Result<ItemHeader, ItemError>
    where
        R: ReadBytesExt,
    {
        // TODO: validation
        Ok(Self {
            item_id: reader.read_u32_le()?,
            domain_id: reader.read_u32_le()?,
            header_flags: reader.read_u32_le()?,
            uuid: reader.read_bytes(16)?,
        })
    }
}
