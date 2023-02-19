/// bool u8 +0x20 ?
/// u32 +0x24 AuthoringApplication
/// Version +0x28 AuthoringApplicationVersion

/// Data field type 101
pub struct Preset {
    authoring_application: AuthoringApplication,
    version: String,
}

enum AuthoringApplication {
    Kontakt,
}

impl Preset {}
