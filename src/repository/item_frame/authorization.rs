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

use crate::{prelude::*, read_bytes::ReadBytesExt, repository::item_frame::ItemFrameHeader};

pub struct Authorization(Vec<u8>);

impl Authorization {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {
        log::debug!("Reading Authorization");

        let header = ItemFrameHeader::read(&mut reader)?;
        log::debug!("ItemFrameHeader: {:?}", &header);

        log::debug!("read");
        // blank item (24 bytes)
        let _ = reader.read_sized_data()?;

        // version == 1
        assert_eq!(reader.read_u32_le()?, 1);

        let a = reader.read_u32_le()?; // 0x18
        if a == 1 {
            // LicenseInfo::read
        }

        Ok(Authorization(vec![]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authorization_read() -> Result<()> {
        crate::utils::setup_logger();

        let path = "tests/data/item-frame/kontakt-4/106-Authorization.data";
        log::info!("reading {:?}", path);

        let file = std::fs::read(&path)?;
        let _auth = Authorization::read(file.as_slice())?;

        // TODO: auth props

        Ok(())
    }
}
