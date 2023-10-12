use crate::{
    nis::{ItemContainer, ItemType, Preset},
    Error,
};

impl ItemContainer {
    pub fn find_preset_item(&self) -> Option<Result<Preset, Error>> {
        self.find_item::<Preset>(&ItemType::Preset)
    }
}
