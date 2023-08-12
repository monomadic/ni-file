use std::convert::TryInto;

use crate::{
    nisound::{item_frame::ItemFrame, ItemID},
    prelude::*,
};

use super::preset::Preset;

/// Kontakt preset
pub struct BNISoundPreset {
    pub preset: Preset,
}

impl std::convert::TryFrom<&ItemFrame> for BNISoundPreset {
    type Error = NIFileError;

    fn try_from(frame: &ItemFrame) -> Result<Self> {
        debug_assert_eq!(frame.header.item_id, ItemID::BNISoundPreset);

        let frame = *frame.inner.clone().unwrap();

        // .. data

        Ok(Self {
            preset: frame.try_into()?,
        })
    }
}
