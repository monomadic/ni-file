use crate::nis::ItemContainer;

#[derive(Debug)]
pub struct Preset(ItemContainer);

impl Preset {
    // pub fn preset(&self) -> Result<?, Error> {
    //     unimplemented!()
    // }

    // pub fn sound_info_item(&self) -> Result<SoundInfoItem, Error> {}

    pub fn children(&self) -> &Vec<ItemContainer> {
        &self.0.children
    }
}

impl From<ItemContainer> for Preset {
    fn from(ic: ItemContainer) -> Self {
        // TODO: checks
        Self(ic)
    }
}
