use std::convert::TryInto;

use crate::{
    nisound::{item_frame::ItemFrame, ItemID},
    prelude::*,
};

use super::subtree_item::SubtreeItem;

/// EncryptionItem
/// Usually a container for compressed presets.
/// Children: SubtreeItem

pub struct EncryptionItem {
    pub subtree: SubtreeItem,
}

impl std::convert::TryFrom<&ItemFrame> for EncryptionItem {
    type Error = NIFileError;

    fn try_from(frame: &ItemFrame) -> Result<Self> {
        log::debug!("BNISoundPreset::try_from");
        debug_assert_eq!(frame.header.item_id, ItemID::EncryptionItem);

        let subtree_frame = ItemFrame::read(frame.inner.0.clone())?;

        // .. data

        Ok(Self {
            subtree: subtree_frame.try_into()?,
        })
    }
}
