// Authorization {
//  item Item,
//  license LicenseInfo {}, // 11 x u32?
//  u32 1,
//  u32 0,
// }
// props:
// - LicenseInfo::importProperties
//   @product-bindings
//   @watermark
// @authorization-level

use crate::{
    prelude::*,
    read_bytes::ReadBytesExt,
    repository::item_frame::{item_id::ItemID, ItemFrame},
};

pub struct Authorization(Vec<u8>);

impl std::convert::TryFrom<ItemFrame> for Authorization {
    type Error = NIFileError;

    fn try_from(frame: ItemFrame) -> std::result::Result<Self, Self::Error> {
        log::debug!("Authorization::try_from");
        debug_assert_eq!(frame.header.item_id, ItemID::Authorization);
        Authorization::read(frame.data.as_slice())
    }
}

impl Authorization {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {
        log::debug!("Authorization::read");

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
    use super::*;

    #[test]
    fn test_authorization_read() -> Result<()> {
        crate::utils::setup_logger();

        let path = "tests/data/item-frame-property/kontakt-5/106-Authorization.data";
        log::info!("reading {:?}", path);

        let file = std::fs::read(&path)?;
        let _auth = Authorization::read(file.as_slice())?;

        // TODO: auth props

        Ok(())
    }
}
