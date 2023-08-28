use crate::NIFileError;

use super::{item_frame::ItemFrame, Domain};

pub struct PresetContainer(ItemFrame);

impl PresetContainer {
    pub fn inner(&self) -> &ItemFrame {
        &self.0
    }
}

impl TryFrom<ItemFrame> for PresetContainer {
    type Error = NIFileError;

    fn try_from(i: ItemFrame) -> Result<Self, Self::Error> {
        if i.header.domain != Domain::NISD {
            return Err(NIFileError::Generic(format!(
                "Item is not a PresetContainer"
            )));
        }
        Ok(Self(i))
    }
}
