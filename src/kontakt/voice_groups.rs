use crate::{
    kontakt::{pubdata::PubData, voice_limit::VoiceLimit},
    read_bytes::ReadBytesExt,
    Error,
};

#[derive(Debug)]
pub struct VoiceGroups;

// 0x32
impl VoiceGroups {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        println!("VoiceGroups::read");

        let _is_chunked = reader.read_bool()?;
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

#[test]
fn test_zone_list() -> Result<(), Error> {
    let file = include_bytes!("tests/voice_groups/4.2.2.4504/000");
    assert!(VoiceGroups::read(file.as_slice()).is_ok());

    // let file = include_bytes!("tests/voice_groups/default/000").as_slice();
    // assert!(VoiceGroups::read(file).is_ok());

    Ok(())
}
