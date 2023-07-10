use crate::{read_bytes::ReadBytesExt, Error};

// VoiceLimit id: 0x2B

#[derive(Debug)]
pub struct VoiceLimit {
    name: String,
    kill_mode: i16,
    prefer_released: bool,
    max_num_voices: i32,
    ms_fade_time: i32,
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
