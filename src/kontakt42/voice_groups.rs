use crate::{read_bytes::ReadBytesExt, Error};

pub struct VoiceGroups;

impl VoiceGroups {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        reader.read_bool()?;
        let _id = reader.read_u16_le()?;
        // pubdata(0x2b, id)
        //
        // 0x2b, 0x60 = VoiceLimit
        Ok(Self {})
    }
}
