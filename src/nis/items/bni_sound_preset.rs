use std::convert::TryInto;

use crate::{
    nis::{item_data::ItemData, ItemID},
    prelude::*,
};

use super::preset::Preset;

/// Kontakt preset
pub struct BNISoundPreset {
    pub preset: Preset,
}

impl std::convert::TryFrom<&ItemData> for BNISoundPreset {
    type Error = NIFileError;

    fn try_from(frame: &ItemData) -> Result<Self> {
        debug_assert_eq!(frame.header.item_id, ItemID::BNISoundPreset);

        let frame = &*frame.inner.clone().unwrap();

        // .. data

        Ok(Self {
            preset: frame.try_into()?,
        })
    }
}
