/*
    BNISoundPreset (0x3, 3, 4KIN)
    kontakt preset

    BNISoundPreset::readItem(&stream, context) {
        let header = ItemFrameReader(&context);
        let preset = Preset::readItem(&stream, &context)?;
        let version = context.read_u16();
        if version != 0 {
            return Err(VERSION_MISMATCH)
        }
    }
*/

use std::convert::TryInto;

use crate::{prelude::*, repository::item_frame::ItemFrame, ItemID};

use super::preset::Preset;

pub struct BNISoundPreset {
    pub preset: Preset,
}

impl std::convert::TryFrom<ItemFrame> for BNISoundPreset {
    type Error = NIFileError;

    fn try_from(frame: ItemFrame) -> Result<Self> {
        log::debug!("BNISoundPreset::try_from");
        debug_assert_eq!(frame.header.item_id, ItemID::BNISoundPreset);

        let preset: Preset = frame.inner()?.try_into()?;

        // .. data

        Ok(Self { preset })
    }
}
