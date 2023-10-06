use crate::{
    nis::{ItemContainer, ItemID, Preset},
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
        if id != &ItemID::Preset {
            return Err(Error::ItemWrapError {
                expected: ItemID::Preset,
                got: id.clone(),
            });
        }
        Ok(Self(container.clone()))
    }
}
