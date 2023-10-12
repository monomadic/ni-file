use crate::{
    nis::{ItemContainer, ItemType, SubtreeItem},
    Error,
};

pub struct SubtreeItemItem(ItemContainer);

impl SubtreeItemItem {
    pub fn properties(&self) -> Result<SubtreeItem, Error> {
        SubtreeItem::try_from(&self.0.data)
    }

    pub fn item(&self) -> Result<ItemContainer, Error> {
        self.properties().and_then(|subtree| subtree.item())
    }
}

impl TryFrom<&ItemContainer> for SubtreeItemItem {
    type Error = Error;

    fn try_from(container: &ItemContainer) -> Result<Self, Self::Error> {
        let id = container.id();
        if id != ItemType::SubtreeItem {
            return Err(Error::ItemWrapError {
                expected: ItemType::SubtreeItem,
                got: id.clone(),
            });
        }
        Ok(Self(container.clone()))
    }
}
