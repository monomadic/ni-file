use crate::nks::header::BPatchHeader;

use super::KontaktPreset;

pub struct KontaktInstrument {
    pub header: BPatchHeader,
    pub preset: KontaktPreset,
    // pub sound_info: SoundInfoItem,
}
