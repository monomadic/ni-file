use crate::nis::ItemContainer;

#[derive(Debug)]
pub struct AppSpecific(ItemContainer);

impl AppSpecific {
    pub fn children(&self) -> &Vec<ItemContainer> {
        &self.0.children
    }
}

impl From<ItemContainer> for AppSpecific {
    fn from(ic: ItemContainer) -> Self {
        Self(ic)
    }
}
