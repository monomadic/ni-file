use crate::{
    kontakt::{bparam_array::BParamArray, voice_groups::VoiceGroups},
    read_bytes::ReadBytesExt,
    Error,
};

use super::{
    objects::program_data::{ProgramDataV80, ProgramDataVA5},
    zone_data::{ZoneDataV95, ZoneDataV98},
};

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum PubData {
    EnvelopeAHDSR_V10,
    EnvelopeAHDSR_V11,
    ZoneListV95(ZoneDataV95),
    ZoneListV98(ZoneDataV98),
    ProgramDataV80(ProgramDataV80),
    ProgramDataVA5(ProgramDataVA5),
    VoiceGroups(VoiceGroups),
    BParamArray(BParamArray),
    Empty,
}

impl PubData {
    pub fn from<R: ReadBytesExt>(mut reader: R, id: u16, version: u16) -> Result<Self, Error> {
        println!("K4PL_PubData::from(0x{:x}, 0x{:x})", id, version);

        // match id {
        //     _ if id > 0x3e => match id {
        //         0x3f => match version {
        //             0x10 => return Ok(EnvelopeAHDSR_V10),
        //             0x11 => return Ok(EnvelopeAHDSR_V11),
        //             _ => {
        //                 panic!("Unknown EnvelopeAHDSR version: {}", version)
        //             }
        //         },
        //         0x41 => {}
        //         _ => panic!(),
        //     },
        //     _ if id < 0x28 => (),
        //     _ => (),
        // }

        if id > 0x3e {
            match id {
                0x3f => match version {
                    0x10 => return Ok(PubData::EnvelopeAHDSR_V10),
                    0x11 => return Ok(PubData::EnvelopeAHDSR_V11),
                    _ => {
                        panic!("Unknown EnvelopeAHDSR version: {}", version)
                    }
                },
                0x45 => {
                    return Ok(PubData::VoiceGroups(VoiceGroups::read(&mut reader)?));
                }
                _ => panic!("pubdata id not supported: 0x{:x}", id),
            }
        }

        if id < 0x28 {
            if id == 0x25 {
                return Ok(PubData::Empty);
            } else {
                panic!("id < 0x28 - 0x{id:x}");
            }
        }

        match id {
            0x28 => match version {
                0x80 => return Ok(PubData::ProgramDataV80(ProgramDataV80::read(&mut reader)?)),
                0xA5 => return Ok(PubData::ProgramDataVA5(ProgramDataVA5::read(&mut reader)?)),
                _ => panic!("Unknown ProgramData version: {}", version),
            },
            0x2c => match version {
                _ if version < 0x96 => Ok(PubData::ZoneListV98(ZoneDataV98::read(&mut reader)?)),
                _ if version < 0x99 => Ok(PubData::ZoneListV95(ZoneDataV95::read(&mut reader)?)),
                _ => panic!("Unknown ZoneData version: {}", version),
            },
            0x32 => Ok(PubData::VoiceGroups(VoiceGroups::read(&mut reader)?)),
            0x3a => Ok(PubData::BParamArray(BParamArray::read(&mut reader, 8)?)),
            _ => {
                panic!("Unsupported PubData id: {id}");
            }
        }
    }
}

#[test]
fn test_pubdata_0x28_0x80() -> Result<(), Error> {
    let mut file = include_bytes!("tests/ProgramData/0x28-0x80").as_slice();
    let _pd = PubData::from(&mut file, 0x28, 0x80)?;
    // println!("{pd:?}");

    Ok(())
}
