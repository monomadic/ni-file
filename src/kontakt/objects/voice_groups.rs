use crate::{kontakt::objects::voice_limit::VoiceLimit, read_bytes::ReadBytesExt, Error};

#[derive(Debug)]
pub struct VoiceGroups;

#[derive(Debug)]
pub struct VoiceGroup;

// SerId 0x32
impl VoiceGroups {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        println!("VoiceGroups::read");

        let is_structured = reader.read_bool()?;
        assert_eq!(is_structured, false);

        let version = reader.read_u16_le()?;

        match version {
            0x60 => println!("{:?}", VoiceLimit::read(&mut reader)?),
            _ => println!("unsupported VoiceGroups version: 0x{:x}", version),
        }

        for i in 0..16 {
            println!("{}: {:?}", i + 1, reader.read_u8()?);
        }

        //println!("{:?}", PubData::create(&mut reader, 0x2B, version)?);

        Ok(Self {})
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn test_voice_groups_v60() -> Result<(), Error> {
        let file = File::open("tests/patchdata/KontaktV42/VoiceGroups/v60/000")?;
        assert!(VoiceGroups::read(file).is_ok());
        Ok(())
    }
}
