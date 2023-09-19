use crate::nis::{Domain, ItemID};
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
        let length: u64 = reader.read_u64_le()?;
        let domain: Domain = reader.read_u32_le()?.into();
        let item_id: ItemID = reader.read_u32_le()?.into();
        let version: u32 = reader.read_u32_le()?;

        if let ItemID::Unknown(id) = item_id {
            return Err(NIFileError::Generic(format!(
                "ItemFrameHeader unexpected ItemID error: got 0x{id:x}"
            )));
        }

        if version != 1 {
            return Err(NIFileError::Generic(format!(
                "ItemFrameHeader version error: expected 0x1, got 0x{version:x}"
            )));
        }

        Ok(Self {
            length,
            domain,
            item_id,
            version,
        })
    }
}
