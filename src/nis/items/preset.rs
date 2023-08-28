// properties
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

use crate::nis::item_frame::{app_id::AuthoringApplication, ItemFrame};
use crate::nis::ItemID;
use crate::prelude::*;
use crate::read_bytes::ReadBytesExt;

pub struct Preset {
    pub is_compressed: bool,
    pub authoring_app: AuthoringApplication,
    pub version: String,
}

impl std::convert::TryFrom<ItemFrame> for Preset {
    type Error = NIFileError;

    fn try_from(frame: ItemFrame) -> Result<Self> {
        debug_assert_eq!(frame.header.item_id, ItemID::Preset);
        Preset::read(Cursor::new(frame.data.clone()))
    }
}

impl Preset {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {
        assert_eq!(reader.read_u32_le()?, 1);

        let is_compressed = reader.read_u8()?;
        log::debug!("is_compressed: {}", is_compressed);

        let authoring_app: AuthoringApplication = reader.read_u32_le()?.into();
        log::debug!("authoring_app_id: {:?}", authoring_app);

        // AuthoringApplicationInfo

        // check ver
        assert_eq!(reader.read_u32_le()?, 1);

        let version = reader.read_widestring_utf16()?;

        Ok(Preset {
            is_compressed: is_compressed == 1,
            authoring_app,
            version,
        })
    }
}
