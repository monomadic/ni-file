/// Data field type 101
pub struct Preset {
    authoring_application: AuthoringApplication,
    version: String,
}

enum AuthoringApplication {
    Kontakt,
}

impl Preset {}
