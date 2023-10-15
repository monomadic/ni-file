use super::{objects::BPatchHeader, KontaktPreset};

#[derive(Debug)]
pub struct KontaktInstrument {
    pub header: BPatchHeader,
    pub preset: KontaktPreset,
    // pub sound_info: SoundInfoItem,
}
