use std::convert::TryInto;

use crate::{
    nis::{ItemData, ItemID},
    prelude::*,
};

use super::subtree_item::SubtreeItem;

/// A container for compressed presets.
pub struct EncryptionItem {
    pub subtree: SubtreeItem,
}

impl std::convert::TryFrom<&ItemData> for EncryptionItem {
    type Error = NIFileError;

    fn try_from(frame: &ItemData) -> Result<Self> {
        debug_assert_eq!(frame.header.item_id, ItemID::EncryptionItem);

        let subtree_frame = &*frame.inner.clone().unwrap();

        // .. data

        Ok(Self {
            subtree: subtree_frame.try_into()?,
        })
    }
}
