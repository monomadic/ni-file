use crate::read_bytes::ReadBytesExt;

use super::ItemError;

// TODO: convert to named type

// pub struct ItemHeader(Vec<u8>);

#[derive(Debug, Clone)]
pub struct ItemHeader {
    pub size: u64,
    pub item_id: u32,      // (+0x8, uint, 0x01000000)
    pub domain_id: u32,    // (+0xC, uint, 'hsin')
    pub header_flags: u32, // (0x10, uint, see [ItemID])
    pub uuid: Vec<u8>,     // (0x14, 16 bytes, randomly generated)
}

impl ItemHeader {
    pub fn read<R>(mut reader: R) -> Result<ItemHeader, ItemError>
    where
        R: ReadBytesExt,
    {
        Ok(Self {
            size: reader.read_u64_le()?,
            item_id: reader.read_u32_le()?,
            domain_id: reader.read_u32_le()?,
            header_flags: reader.read_u32_le()?,
            uuid: reader.read_bytes(16)?,
        })
    }
}
