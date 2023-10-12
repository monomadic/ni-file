use std::convert::TryInto;

use crate::{
    nis::{ItemData, ItemType},
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
        debug_assert_eq!(frame.header.item_type(), ItemType::BNISoundPreset);

        let frame = &*frame.inner.clone().unwrap();

        Ok(Self {
            preset: frame.try_into()?,
        })
    }
}
