use crate::nis::ItemContainer;

#[derive(Debug)]
pub struct BNISoundPreset(ItemContainer);

impl BNISoundPreset {
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
