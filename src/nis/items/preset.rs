use crate::{
    nis::{ItemContainer, ItemType, Preset},
    Error,
};

#[derive(Debug)]
pub struct PresetContainer(ItemContainer);

impl PresetContainer {
    pub fn properties(&self) -> Result<Preset, Error> {
        Preset::try_from(&self.0.data)
    }
}

impl TryFrom<&ItemContainer> for PresetContainer {
    type Error = Error;

    fn try_from(container: &ItemContainer) -> Result<Self, Self::Error> {
        let id = container.id();
        if id != ItemType::Preset {
            return Err(Error::ItemWrapError {
                expected: ItemType::Preset,
                got: id.clone(),
            });
        }
        Ok(Self(container.clone()))
    }
}
