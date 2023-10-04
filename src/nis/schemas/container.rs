// TODO: remove
//
use crate::{
    nis::{Domain, ItemData},
    NIFileError,
};

#[derive(Debug)]
pub struct Container(ItemData);

impl Container {
    pub fn inner(&self) -> &ItemData {
        &self.0
    }

    // pub fn detect(&self) -> NISSchema {}
}

impl TryFrom<ItemData> for Container {
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
