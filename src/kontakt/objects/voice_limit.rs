use crate::{read_bytes::ReadBytesExt, Error};

// BVoiceLimit id: 0x2B
#[derive(Debug)]
pub struct VoiceLimit {
    name: String,
    /// Method to decide which voices will be killed.
    /// - Options: Any, Oldest, Newest, Highest, Lowest
    /// - Default: Oldest
    kill_mode: i16,
    /// Prefer to keep already released voices.
    /// - Default: true
    prefer_released: bool,
    /// Maximum number of voices that can be used by this voice group.
    /// - Default: 1
    max_num_voices: i32,
    /// Time in ms for stolen voices to fade out.
    /// - Default: 10
    ms_fade_time: i32,
    /// Kills playing samples in other exclusion groups.
    /// - Range: 1-16, off (0)
    /// - Default: off
    exclusion_group: i32,
}

impl VoiceLimit {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        Ok(Self {
            name: reader.read_widestring_utf16()?,
            kill_mode: reader.read_i16_le()?,
            prefer_released: reader.read_bool()?,
            max_num_voices: reader.read_i32_le()?,
            ms_fade_time: reader.read_i32_le()?,
            exclusion_group: reader.read_i32_le()?,
        })
    }
}
