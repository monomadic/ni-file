use crate::{kontakt::pubdata::PubData, read_bytes::ReadBytesExt, Error};

#[derive(Debug)]
pub struct VoiceGroups;

// 0x32
impl VoiceGroups {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        reader.read_bool()?;
        let version = reader.read_u16_le()?;
        // println!("{:?}", PubData::create(&mut reader, 0x32, version)?);
        Ok(Self {})
    }
}

#[test]
fn test_zone_list() -> Result<(), Error> {
    let file = include_bytes!("tests/voice_groups/4.2.2.4504/000");
    assert!(VoiceGroups::read(file.as_slice()).is_ok());
    Ok(())
}
