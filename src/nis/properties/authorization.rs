// Authorization
//
// Properties
// - LicenseInfo
// - authorizaton-level

use std::io::Cursor;

use crate::{
    nis::{ItemData, ItemType},
    read_bytes::ReadBytesExt,
    NIFileError,
};

#[derive(Debug)]
pub struct Authorization {
    level: u32,
    read_checksum: u32,
}

impl std::convert::TryFrom<&ItemData> for Authorization {
    type Error = NIFileError;

    fn try_from(frame: &ItemData) -> std::result::Result<Self, Self::Error> {
        debug_assert_eq!(frame.header.item_type(), ItemType::Authorization);
        Authorization::read(Cursor::new(&frame.data))
    }
}

impl Authorization {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, NIFileError> {
        // version == 1
        assert_eq!(reader.read_u32_le()?, 1);

        let a = reader.read_u32_le()?; // 0x18
        if a == 1 {
            // LicenseInfo::read
            let num_snpids = reader.read_u32_le()?;
            for _ in 0..num_snpids {
                let snp_id = reader.read_widestring_utf16()?;
                assert_eq!(snp_id.len(), 0);
            }
        }

        reader.read_u32_le()?;

        // authorizationLevel
        let level = reader.read_u32_le()?; // 24, default: 1
        let read_checksum = reader.read_u32_le()?; // 28, default: 0

        Ok(Authorization {
            level,
            read_checksum,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn test_authorization_read() -> Result<(), NIFileError> {
        let file = File::open(
            "tests/data/nisound/chunks/item-frame-property/kontakt-5/106-Authorization.data",
        )?;

        let _auth = Authorization::read(&file)?;

        Ok(())
    }
}
