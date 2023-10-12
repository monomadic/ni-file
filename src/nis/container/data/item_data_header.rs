use crate::{nis::ItemType, read_bytes::ReadBytesExt, NIFileError};

/// 20 bytes
#[derive(Debug, Clone)]
pub struct ItemDataHeader {
    pub length: u64,
    pub domain_id: [u8; 4],
    pub item_id: u32,
    pub version: u32,
}

impl ItemDataHeader {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, NIFileError> {
        let length: u64 = reader.read_u64_le()?;
        let mut domain_id = [0; 4];
        reader.read_exact(&mut domain_id)?;
        let item_id: u32 = reader.read_u32_le()?;
        let version: u32 = reader.read_u32_le()?;

        // Swap to match little-endian representation
        domain_id.reverse();

        if version != 1 {
            return Err(NIFileError::VersionMismatch {
                expected: 1,
                got: version,
            });
        }

        Ok(Self {
            length,
            domain_id,
            item_id,
            version,
        })
    }

    pub fn item_type(&self) -> ItemType {
        let domain_id = std::str::from_utf8(&self.domain_id).expect("Not UTF-8");
        ItemType::new(self.item_id, domain_id)
    }
}
