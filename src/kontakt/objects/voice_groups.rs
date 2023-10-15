use std::io::Cursor;

use crate::{
    kontakt::{objects::voice_limit::VoiceLimit, Chunk, KontaktError},
    read_bytes::ReadBytesExt,
    Error,
};

const MAX_VOICE_GROUPS: usize = 128;

#[derive(Debug)]
pub struct VoiceGroups {
    voice_limit: VoiceLimit,
    groups: Vec<Option<VoiceGroup>>,
}

#[derive(Debug)]
pub struct VoiceGroup;

/// ID 0x32
impl VoiceGroups {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let is_structured = reader.read_bool()?;
        let version = reader.read_u16_le()?;

        assert_eq!(is_structured, false);

        let voice_limit = match version {
            0x60 => VoiceLimit::read(&mut reader)?,
            _ => unimplemented!("Unsupported VoiceGroups version: 0x{:x}", version),
        };

        let indexes = reader.read_bytes(8)?;

        // let mut groups: [Option<VoiceGroup>; MAX_VOICE_GROUPS] = [None; MAX_VOICE_GROUPS];
        let groups = Vec::new();
        for i in 0..MAX_VOICE_GROUPS {
            if indexes[i >> 3] & (1 << (i & 7)) != 0 {
                // groups[i] = None;
                // println!("{}: {:?}", i + 1, reader.read_u8()?);
            }
        }

        // next

        Ok(Self {
            voice_limit,
            groups,
        })
    }
}

impl std::convert::TryFrom<&Chunk> for VoiceGroups {
    type Error = Error;

    fn try_from(chunk: &Chunk) -> Result<Self, Self::Error> {
        if chunk.id != 0x32 {
            return Err(KontaktError::IncorrectID {
                expected: 0x32,
                got: chunk.id,
            }
            .into());
        }
        let reader = Cursor::new(&chunk.data);
        Self::read(reader)
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use super::*;

    #[test]
    fn test_voice_groups_v60() -> Result<(), Error> {
        let mut file = File::open("tests/data/Objects/Kontakt/VoiceGroups/v60/000")?;

        VoiceGroups::read(&mut file)?;

        // Ensure the read completed
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        assert_eq!(buf.len(), 0, "Excess data found: {} bytes", buf.len());

        Ok(())
    }
}
