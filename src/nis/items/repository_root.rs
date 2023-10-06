use crate::{
    nis::{kontakt::BNISoundPresetContainer, ItemContainer, ItemID, RepositoryRoot},
    Error,
};

use super::{AppSpecificItem, PresetContainer};

pub struct RepositoryRootContainer(pub ItemContainer);

impl RepositoryRootContainer {
    pub fn properties(&self) -> Result<RepositoryRoot, Error> {
        RepositoryRoot::try_from(&self.0.data)
    }

    pub fn kontakt_preset(&self) -> Option<Result<BNISoundPresetContainer, Error>> {
        self.0
            .find(&ItemID::BNISoundPreset)
            .map(BNISoundPresetContainer::try_from)
    }

    pub fn preset(&self) -> Option<Result<PresetContainer, Error>> {
        self.0.find(&ItemID::Preset).map(PresetContainer::try_from)
    }

    pub fn app_specific(&self) -> Option<Result<AppSpecificItem, Error>> {
        self.0
            .find(&ItemID::AppSpecific)
            .map(AppSpecificItem::try_from)
    }
}

impl TryFrom<&ItemContainer> for RepositoryRootContainer {
    type Error = Error;

    fn try_from(container: &ItemContainer) -> Result<Self, Self::Error> {
        let id = container.id();
        if id != &ItemID::RepositoryRoot {
            return Err(Error::ItemWrapError {
                expected: ItemID::RepositoryRoot,
                got: id.clone(),
            });
        }
        Ok(Self(container.clone()))
    }
}
