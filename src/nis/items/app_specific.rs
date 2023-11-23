use crate::{
    nis::{AppSpecificProperties, ItemContainer, ItemType},
    Error,
};

use super::SubtreeItemItem;

pub struct AppSpecificItem(ItemContainer);

impl AppSpecificItem {
    pub fn properties(&self) -> Result<AppSpecificProperties, Error> {
        AppSpecificProperties::try_from(&self.0.data)
    }

    pub fn subtree_item(&self) -> Option<Result<SubtreeItemItem, Error>> {
        self.0
            .find(&ItemType::SubtreeItem)
            .map(SubtreeItemItem::try_from)
    }
}

impl TryFrom<&ItemContainer> for AppSpecificItem {
    type Error = Error;

    fn try_from(container: &ItemContainer) -> Result<Self, Self::Error> {
        let id = container.id();
        if id != ItemType::AppSpecific {
            return Err(Error::ItemWrapError {
                expected: ItemType::AppSpecific,
                got: id.clone(),
            });
        }
        Ok(Self(container.clone()))
    }
}
