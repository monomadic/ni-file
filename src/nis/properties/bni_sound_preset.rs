use std::convert::TryInto;

use crate::{
    nis::{ItemData, ItemID},
    prelude::*,
};

use super::preset::Preset;

/// Wraps a Kontakt preset
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
