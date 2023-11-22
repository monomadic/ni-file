use std::io::Cursor;

use crate::Error;

use super::{objects::BPatchHeaderV42, schemas::KontaktPreset};

#[derive(Debug)]
pub struct KontaktPatch {
    pub header: BPatchHeaderV42,
    pub data: Vec<u8>,
    // pub sound_info: SoundInfoItem,
}

impl KontaktPatch {
    pub fn preset(&self) -> Result<KontaktPreset, Error> {
        KontaktPreset::read(
            Cursor::new(&self.data),
            &self.header.app_signature,
            &self.header.patch_type,
            &self.header.min_supported_version,
        )
    }
}
