use crate::{
    nis::{kontakt::BNISoundPresetContainer, ItemContainer, ItemType, RepositoryRoot},
    Error, NIFileError,
};

use super::{AppSpecificItem, PresetContainer};

pub struct RepositoryRootContainer(pub ItemContainer);

impl RepositoryRootContainer {
    pub fn properties(&self) -> Result<RepositoryRoot, Error> {
        RepositoryRoot::try_from(&self.0.data)
    }

    /// Attempt to extract an inner preset from any kind of NISound document.
    pub fn extract_preset(&self) -> Result<Vec<u8>, Error> {
        // Check for generic preset
        if let Some(preset) = self.preset() {
            if let Some(preset) = preset?.preset_data() {
                return Ok(preset?);
            }
        }

        // Check for Kontakt NIS
        if let Some(preset) = self.kontakt_preset() {
            if let Some(preset) = preset?.preset_data() {
                return Ok(preset?);
            }
        }

        // Check for Kontakt NKM
        if let Some(app_specific) = self.app_specific() {
            let app = app_specific?;
            let props = app.properties()?;
            let inner = RepositoryRootContainer(props.subtree_item.item()?);

            if let Some(preset) = inner.kontakt_preset() {
                if let Some(preset) = preset?.preset_data() {
                    return Ok(preset?);
                }
            }
        }

        Err(NIFileError::Generic("Could not find a valid preset".into()))
    }

    pub fn kontakt_preset(&self) -> Option<Result<BNISoundPresetContainer, Error>> {
        self.0
            .find(&ItemType::BNISoundPreset)
            .map(BNISoundPresetContainer::try_from)
    }

    pub fn preset(&self) -> Option<Result<PresetContainer, Error>> {
        self.0
            .find(&ItemType::Preset)
            .map(PresetContainer::try_from)
    }

    pub fn app_specific(&self) -> Option<Result<AppSpecificItem, Error>> {
        self.0
            .find(&ItemType::AppSpecific)
            .map(AppSpecificItem::try_from)
    }
}

impl TryFrom<&ItemContainer> for RepositoryRootContainer {
    type Error = Error;

    fn try_from(container: &ItemContainer) -> Result<Self, Self::Error> {
        let id = container.id();
        if id != ItemType::RepositoryRoot {
            return Err(Error::ItemWrapError {
                expected: ItemType::RepositoryRoot,
                got: id.clone(),
            });
        }
        Ok(Self(container.clone()))
    }
}
