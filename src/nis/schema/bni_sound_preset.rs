use crate::{
    kontakt::{objects::BPatchHeaderV42, KontaktPatch},
    nis::ItemContainer,
    Error,
};

#[derive(Debug)]
pub struct BNISoundPreset(ItemContainer);

impl BNISoundPreset {
    pub fn header(&self) -> Result<BPatchHeaderV42, Error> {
        unimplemented!()
    }

    pub fn preset(&self) -> Result<KontaktPatch, Error> {
        unimplemented!()
    }

    // pub fn sound_info_item(&self) -> Result<SoundInfoItem, Error> {}

    pub fn children(&self) -> &Vec<ItemContainer> {
        &self.0.children
    }
}

impl From<ItemContainer> for BNISoundPreset {
    fn from(ic: ItemContainer) -> Self {
        // TODO: checks
        Self(ic)
    }
}
