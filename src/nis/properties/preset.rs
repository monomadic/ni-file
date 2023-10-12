// Properties
// - ni_factory_flag
// - authoring-app
// - authoring-app-version
//
//
//
// bool u8 +0x20 ?
// u32 +0x24
// Version +0x28
// authoring-app 0x24
// authoring-app-version 0x28
// ItemFrame 101: Preset
// pub struct Preset {
//     authoring_app_id: u32,
//     authoring_app_version: String,
// }

use std::io::Cursor;

use crate::nis::{ItemData, ItemID};
use crate::prelude::*;
use crate::read_bytes::ReadBytesExt;

pub struct Preset {
    pub is_factory_preset: bool,
    pub authoring_app: AuthoringApplication,
    pub version: String,
}

#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum AuthoringApplication {
    GuitarRig,
    Kontakt,
    Kore,
    Reaktor,
    Maschine,
    Absynthe,
    Massive,
    FM8,
    Battery,
    KKontrol,
    SC,
    FXF_01,
    FXF_02,
    FXF_03,
    FXF_04,
    FXF_05,
    FXF_06,
    FXF_07,
    FXF_08,
    FXF_09,
    FXF_10,
    FXF_11,
    FXF_12,
    FXF_13,
    FXF_14,
    FXF_15,
    FXF_16,
    FXF_17,
    FXF_18,
    FXF_19,
    Traktor,
    Unknown(u32),
}

impl Preset {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {
        assert_eq!(reader.read_u32_le()?, 1);

        let is_factory_preset = reader.read_bool()?;
        let authoring_app: AuthoringApplication = reader.read_u32_le()?.into();

        assert_eq!(reader.read_u32_le()?, 1);

        let version = reader.read_widestring_utf16()?;

        Ok(Preset {
            is_factory_preset,
            authoring_app,
            version,
        })
    }
}

impl std::convert::TryFrom<&ItemData> for Preset {
    type Error = NIFileError;

    fn try_from(frame: &ItemData) -> Result<Self> {
        debug_assert_eq!(frame.header.item_id, ItemID::Preset);
        Preset::read(Cursor::new(frame.data.clone()))
    }
}

impl From<u32> for AuthoringApplication {
    fn from(app_id: u32) -> Self {
        match app_id {
            0x1 => AuthoringApplication::GuitarRig,
            0x2 => AuthoringApplication::Kontakt,
            0x3 => AuthoringApplication::Kore,
            0x4 => AuthoringApplication::Reaktor,
            0x5 => AuthoringApplication::Maschine,
            0x6 => AuthoringApplication::Absynthe,
            0x7 => AuthoringApplication::Massive,
            0x8 => AuthoringApplication::FM8,
            0x9 => AuthoringApplication::Battery,
            0xA => AuthoringApplication::KKontrol,
            0xB => AuthoringApplication::SC,
            0xC => AuthoringApplication::FXF_01,
            0xD => AuthoringApplication::FXF_02,
            0xE => AuthoringApplication::FXF_03,
            0xF => AuthoringApplication::FXF_04,
            0x10 => AuthoringApplication::FXF_05,
            0x11 => AuthoringApplication::FXF_06,
            0x12 => AuthoringApplication::FXF_07,
            0x13 => AuthoringApplication::FXF_08,
            0x14 => AuthoringApplication::FXF_09,
            0x15 => AuthoringApplication::FXF_10,
            0x16 => AuthoringApplication::FXF_11,
            0x17 => AuthoringApplication::FXF_12,
            0x18 => AuthoringApplication::FXF_13,
            0x19 => AuthoringApplication::FXF_14,
            0x1A => AuthoringApplication::FXF_15,
            0x1B => AuthoringApplication::FXF_16,
            0x1C => AuthoringApplication::FXF_17,
            0x1D => AuthoringApplication::FXF_18,
            0x1E => AuthoringApplication::FXF_19,
            0x1F => AuthoringApplication::Traktor,
            _ => AuthoringApplication::Unknown(app_id),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn test_app_specific_read() -> Result<()> {
        let file = File::open("test-data/NIS/properties/Preset/Preset-000")?;
        let item = Preset::read(file)?;

        assert!(!item.is_factory_preset);
        assert_eq!(item.authoring_app, AuthoringApplication::Kontakt);
        assert_eq!(item.version, String::from("7.1.3.0"));
        Ok(())
    }
}
