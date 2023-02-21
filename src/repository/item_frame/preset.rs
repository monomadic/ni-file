/// bool u8 +0x20 ?
/// u32 +0x24
/// Version +0x28
// authoring-app 0x24
// authoring-app-version 0x28

/// ItemFrame 101: Preset
pub struct Preset {
    authoring_app_id: u32,
    authoring_app_version: String,
}

impl Preset {}
