use crate::{read_bytes::ReadBytesExt, Error};

pub struct VoiceLimit;

impl VoiceLimit {
    pub fn read<R: ReadBytesExt>(_reader: R) -> Result<Self, Error> {
        Ok(Self {})
    }
}
