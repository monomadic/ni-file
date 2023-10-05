use crate::nks::header::BPatchHeader;

use super::KontaktPreset;

#[derive(Debug)]
pub struct KontaktInstrument {
    pub header: BPatchHeader,
    pub preset: KontaktPreset,
    // pub sound_info: SoundInfoItem,
}
