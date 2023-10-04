use crate::{
    nis::{BNISoundHeader, BNISoundPreset, EncryptionItem, ItemContainer, ItemID, Preset},
    nks::header::BPatchHeaderV42,
    NIFileError,
};

#[derive(Debug)]
pub struct KontaktPresetSchema(ItemContainer);

impl From<&ItemContainer> for KontaktPresetSchema {
    fn from(ic: &ItemContainer) -> Self {
        Self(ic.clone())
    }
}

impl KontaktPresetSchema {
    pub fn header(&self) -> Result<BPatchHeaderV42, NIFileError> {
        self.0
            .find(&ItemID::BNISoundHeader)
            .ok_or(NIFileError::Static("missing"))
            .and_then(|item_data| BNISoundHeader::try_from(item_data).map(|sh| sh.0))
    }

    pub fn preset_item(&self) -> Result<Preset, NIFileError> {
        self.0
            .find(&ItemID::BNISoundPreset)
            .ok_or(NIFileError::Static("missing"))
            .and_then(|item_data| BNISoundPreset::try_from(item_data).map(|sh| sh.preset))
    }

    pub fn preset_data(&self) -> Result<Vec<u8>, NIFileError> {
        self.0
            .find(&ItemID::EncryptionItem)
            .ok_or(NIFileError::Static("missing"))
            .and_then(|item_data| {
                EncryptionItem::try_from(item_data).map(|sh| sh.subtree.inner_data)
            })
    }

    // pub fn sound_info(&self) -> Result<SoundInfoItem, NIFileError> {
    //     self.0
    //         .find(&ItemID::BNISoundHeader)
    //         .ok_or(NIFileError::Static("missing"))
    //         .and_then(|item_data| BNISoundHeader::try_from(item_data).map(|sh| sh.0))
    // }
}
