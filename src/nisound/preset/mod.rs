//! NIPresets
//! Presets stored inside NIContainers

mod kontakt;

#[derive(Debug)]
enum NIPreset {
    Kontakt,
    Massive,
    Unknown,
}

impl From<u16> for NIPreset {
    fn from(value: u16) -> Self {
        use NIPreset::*;

        match value {
            0x2800 => Kontakt,
            0x1700 => Massive,
            _ => Unknown,
        }
    }
}
