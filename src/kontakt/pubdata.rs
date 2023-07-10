use crate::{
    kontakt::{structured_object::StructuredObject, voice_groups::VoiceGroups},
    read_bytes::ReadBytesExt,
    Error,
};

use super::{
    program_data::{ProgramDataV80, ProgramDataVA5},
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
}

impl PubData {
    pub fn create<R: ReadBytesExt>(mut reader: R, id: u16, version: u16) -> Result<Self, Error> {
        println!("K4PL_PubData::create(0x{:x}, 0x{:x})", id, version);

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
                // 0x45 => {
                //     VoiceGroups::read(&mut reader)?;
                // }
                _ => panic!("pubdata id not supported: 0x{:x}", id),
            }
        }

        if id < 0x28 {
            panic!("id < 0x28");
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
            _ => {
                panic!("id");
            }
        }
    }
}
