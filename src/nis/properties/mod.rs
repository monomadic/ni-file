//! Kinds of [`Item`](crate::nisound::Item) - the building blocks of NISound containers.

mod app_specific;
pub use app_specific::AppSpecificProperties;

pub mod audio_sample_item;

mod authorization;
pub use authorization::Authorization;

pub mod automation_parameters;
pub mod bank;
pub mod bank_container;
pub mod binary_chunk_item;

mod bni_sound_header;
pub use bni_sound_header::BNISoundHeader;

mod bni_sound_preset;
pub use bni_sound_preset::BNISoundPresetProperties;

pub mod controller_assignments;

mod encryption_item;
pub use encryption_item::EncryptionItem;

pub mod external_file_reference;
pub mod internal_resource_reference_item;
pub mod module;
pub mod module_bank;
pub mod picture_item;
pub mod preset_container;
pub mod resources;
pub mod sound_info_item;

mod subtree_item;
pub use subtree_item::SubtreeItem;

mod preset_chunk_item;
pub use preset_chunk_item::PresetChunkItemProperties;

mod preset;
pub use preset::{AuthoringApplication, Preset};

mod repository_root;
pub use repository_root::*;
