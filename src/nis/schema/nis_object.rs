use crate::nis::{ItemContainer, ItemType};

use super::{AppSpecific, BNISoundPreset, Preset, PresetChunkItem, Repository};

pub enum NISObject {
    AppSpecific(AppSpecific),
    Repository(Repository),
    BNISoundPreset(BNISoundPreset),
    Preset(Preset),
    PresetChunkItem(PresetChunkItem),
    Unknown,
}

impl From<&ItemContainer> for NISObject {
    fn from(ic: &ItemContainer) -> Self {
        match ic.data.header.item_type() {
            ItemType::AppSpecific => NISObject::AppSpecific(ic.clone().into()),
            ItemType::BNISoundPreset => NISObject::BNISoundPreset(ic.clone().into()),
            ItemType::Preset => NISObject::Preset(ic.clone().into()),
            ItemType::PresetChunkItem => NISObject::PresetChunkItem(ic.clone().into()),
            ItemType::RepositoryRoot => NISObject::Repository(ic.clone().into()),
            _ => NISObject::Unknown,
        }
    }
}
