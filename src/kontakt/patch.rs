use std::io::Cursor;

use crate::Error;

use super::{
    objects::{BPatchHeaderV42, Program},
    schemas::KontaktPreset,
};

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

    pub fn program(&self) -> Result<Option<Program>, Error> {
        Ok(match self.preset()? {
            KontaktPreset::KontaktV1(_) | KontaktPreset::KontaktV2(_) => None,
            KontaktPreset::KontaktV42(p) => Some(p.program),
            KontaktPreset::Kon5(p) => Some(p.program),
            KontaktPreset::Kon6(p) => Some(p.program),
            KontaktPreset::Kon7(p) => Some(p.program),
            KontaktPreset::NKM(_) => None,
            KontaktPreset::Unsupported(_) => None,
        })
    }
}
