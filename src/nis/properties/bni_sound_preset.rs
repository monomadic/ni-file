use std::convert::TryInto;

use crate::{
    nis::{ItemData, ItemType},
    NIFileError,
};

use super::preset::Preset;

/// Wraps a Preset as an NISoundPreset so extra header and
/// soundinfoitem data can be included as children of the Preset.
/// Only used in Kontakt (so far?).
pub struct BNISoundPresetProperties {
    pub preset: Preset,
}

impl std::convert::TryFrom<&ItemData> for BNISoundPresetProperties {
    type Error = NIFileError;

    fn try_from(frame: &ItemData) -> Result<Self, Self::Error> {
        debug_assert_eq!(frame.header.item_type(), ItemType::BNISoundPreset);

        let frame = &*frame.inner.clone().unwrap();

        Ok(Self {
            preset: frame.try_into()?,
        })
    }
}
