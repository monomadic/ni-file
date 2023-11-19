use crate::nis::{ItemContainer, ItemType};

use super::{BNISoundPreset, Repository};

pub enum NISObject {
    Repository(Repository),
    BNISoundPreset(BNISoundPreset),
    // Preset(Preset),
    Unknown,
}

impl From<&ItemContainer> for NISObject {
    fn from(ic: &ItemContainer) -> Self {
        match ic.data.header.item_type() {
            // ItemType::AppSpecific => NISObject::AppSpecific(self),
            ItemType::BNISoundPreset => NISObject::BNISoundPreset(ic.clone().into()),
            ItemType::RepositoryRoot => NISObject::Repository(ic.clone().into()),
            _ => NISObject::Unknown,
        }
    }
}
