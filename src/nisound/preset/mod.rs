//! NISPreset
//! Presets stored inside NIContainers

mod kontakt;

/// NISPreset
#[derive(Debug)]
enum NISPreset {
    Kontakt,
    Massive,
    Unknown,
}

impl From<u16> for NISPreset {
    fn from(value: u16) -> Self {
        use NISPreset::*;

        match value {
            0x2800 => Kontakt,
            0x1700 => Massive,
            _ => Unknown,
        }
    }
}
