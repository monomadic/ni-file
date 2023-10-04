use crate::{
    nis::{Domain, ItemContainer, ItemData},
    NIFileError,
};

#[derive(Debug)]
pub struct PresetContainer(ItemContainer);

impl PresetContainer {
    pub fn inner(&self) -> &ItemContainer {
        &self.0
    }
}
