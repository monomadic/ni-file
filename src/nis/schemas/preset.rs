use crate::{
    nis::{ItemContainer, ItemID, Preset},
    Error,
};

impl ItemContainer {
    pub fn find_preset_item(&self) -> Option<Result<Preset, Error>> {
        self.find_item::<Preset>(&ItemID::Preset)
    }
}
