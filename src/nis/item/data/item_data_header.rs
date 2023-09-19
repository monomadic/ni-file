use crate::nis::{Domain, ItemID};
use crate::read_bytes::ReadBytesExt;
use crate::NIFileError;

/// 20 bytes
#[derive(Debug, Clone)]
pub struct ItemDataHeader {
    pub length: u64,
    pub domain: Domain,
    pub item_id: ItemID,
    pub version: u32,
}

impl ItemDataHeader {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, NIFileError> {
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
