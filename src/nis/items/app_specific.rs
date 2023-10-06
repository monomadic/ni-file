use crate::{
    nis::{AppSpecific, ItemContainer, ItemID},
    Error,
};

use super::SubtreeItemItem;

pub struct AppSpecificItem(ItemContainer);

impl AppSpecificItem {
    pub fn properties(&self) -> Result<AppSpecific, Error> {
        AppSpecific::try_from(&self.0.data)
    }

    pub fn subtree_item(&self) -> Option<Result<SubtreeItemItem, Error>> {
        self.0
            .find(&ItemID::SubtreeItem)
            .map(SubtreeItemItem::try_from)
    }
}

impl TryFrom<&ItemContainer> for AppSpecificItem {
    type Error = Error;

    fn try_from(container: &ItemContainer) -> Result<Self, Self::Error> {
        let id = container.id();
        if id != &ItemID::AppSpecific {
            return Err(Error::ItemWrapError {
                expected: ItemID::AppSpecific,
                got: id.clone(),
            });
        }
        Ok(Self(container.clone()))
    }
}
