/// bool u8 +0x20 ?
/// u32 +0x24
/// Version +0x28
// authoring-app 0x24
// authoring-app-version 0x28
use super::app_id::AuthoringApplication;

/// Data field type 101
pub struct Preset {
    authoring_application: AuthoringApplication,
    version: String,
}

impl Preset {}
