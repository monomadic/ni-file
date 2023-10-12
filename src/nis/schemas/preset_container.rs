use crate::nis::ItemContainer;

#[derive(Debug)]
pub struct PresetContainer(ItemContainer);

impl PresetContainer {
    pub fn inner(&self) -> &ItemContainer {
        &self.0
    }
}
