/*
    Preset (0x67, 101)

    Preset.readItem(&stream) {
        let header = ItemFrameReader(&stream);
        let auth = Authorization::readItem(&stream)?;

        if stream.read_u32 != 1 {
            return Err(VERSION_MISMATCH);
        }

        let unk[0x20] = stream.read_bool();
        let unk[0x24] = stream.read_u32();
        let unk[0x28] = AuthoringApplicationInfo::readVersion(&stream)
            .map_err(INTERNAL_ERROR)?;
    }

*/

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

use crate::prelude::*;
use crate::read_bytes::ReadBytesExt;

pub struct Preset(Vec<u8>);

impl Preset {
    fn read(&self) -> Result<Preset, NIFileError> {
        let mut buf = self.0.as_slice();

        let prop_version = buf.read_u32_le()?;
        assert_eq!(prop_version, 1);

        let is_compressed = buf.read_u8()?;
        log::debug!("is_compressed: {}", is_compressed);

        todo!();
        Ok(Preset(vec![]))
    }
}
