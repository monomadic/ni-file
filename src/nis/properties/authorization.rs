// Authorization
//
// Properties
// - LicenseInfo
// - authorizaton-level

use std::io::Cursor;

use crate::{
    nis::{ItemData, ItemID},
    prelude::*,
    read_bytes::ReadBytesExt,
};

pub struct Authorization(Vec<u8>);

impl std::convert::TryFrom<&ItemData> for Authorization {
    type Error = NIFileError;

    fn try_from(frame: &ItemData) -> std::result::Result<Self, Self::Error> {
        debug_assert_eq!(frame.header.item_id, ItemID::Authorization);
        Authorization::read(Cursor::new(&frame.data))
    }
}

impl Authorization {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {
        // version == 1
        assert_eq!(reader.read_u32_le()?, 1);

        let a = reader.read_u32_le()?; // 0x18
        if a == 1 {
            // LicenseInfo::read
        }

        // authorizationLevel
        let _authorization_level = reader.read_u32_le()?; // 24, default: 1
        let _read_checksum = reader.read_u32_le()?; // 28, default: 0

        Ok(Authorization(vec![]))
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn test_authorization_read() -> Result<()> {
        let file = File::open(
            "tests/data/nisound/chunks/item-frame-property/kontakt-5/106-Authorization.data",
        )?;

        let _auth = Authorization::read(file)?;

        // TODO: auth props

        Ok(())
    }
}
