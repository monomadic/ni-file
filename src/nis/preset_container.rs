use crate::NIFileError;

use super::{item_data::ItemData, Domain};

pub struct PresetContainer(ItemData);

impl PresetContainer {
    pub fn inner(&self) -> &ItemData {
        &self.0
    }
}

impl TryFrom<ItemData> for PresetContainer {
    type Error = NIFileError;

    fn try_from(i: ItemData) -> Result<Self, Self::Error> {
        if i.header.domain != Domain::NISD {
            return Err(NIFileError::Generic(format!(
                "Item is not a PresetContainer"
            )));
        }
        Ok(Self(i))
    }
}
