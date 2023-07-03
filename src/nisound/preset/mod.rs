//! NISPreset
//! Presets stored deep inside NISound documents

//mod kontakt;

/// NISPreset
#[derive(Debug)]
enum NISPreset {
    Kontakt,
    Massive,
    Unknown(u16),
}

impl From<u16> for NISPreset {
    fn from(value: u16) -> Self {
        use NISPreset::*;

        match value {
            0x2800 => Kontakt,
            0x1700 => Massive,
            _ => Unknown(value),
        }
    }
}
