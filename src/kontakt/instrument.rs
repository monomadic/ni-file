use super::{objects::BPatchHeader, schemas::KontaktPreset};

#[deprecated]
#[derive(Debug)]
pub struct KontaktInstrument {
    pub header: BPatchHeader,
    pub preset: KontaktPreset,
    // pub sound_info: SoundInfoItem,
}
