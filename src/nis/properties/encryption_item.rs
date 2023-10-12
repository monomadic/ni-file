use std::{convert::TryInto, io::Cursor};

use crate::{
    nis::{ItemData, ItemType},
    prelude::*,
};

use super::subtree_item::SubtreeItem;

/// A container for compressed presets.
#[derive(Debug)]
pub struct EncryptionItem {
    pub subtree: SubtreeItem,
    pub is_encrypted: bool,
}

impl std::convert::TryFrom<&ItemData> for EncryptionItem {
    type Error = NIFileError;

    fn try_from(frame: &ItemData) -> Result<Self> {
        debug_assert_eq!(frame.header.item_type(), ItemType::EncryptionItem);

        let subtree_frame = &*frame.inner.clone().unwrap();

        let mut reader = Cursor::new(&frame.data);
        assert_eq!(reader.read_u32_le()?, 1); // version?

        let is_encrypted = reader.read_u8()? == 1;

        Ok(Self {
            subtree: subtree_frame.try_into()?,
            is_encrypted,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use super::*;

    #[test]
    fn test_authorization_read() -> Result<()> {
        let mut file =
            File::open("tests/data/Containers/NIS/objects/EncryptionItem/000-EncryptionItem")?;

        let item = ItemData::read(&file)?;
        let enc = EncryptionItem::try_from(&item)?;

        // ensure the read completed
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        assert_eq!(buf.len(), 0, "Excess data found");

        Ok(())
    }
}
