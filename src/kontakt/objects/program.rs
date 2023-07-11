use crate::{
    kontakt::{
        bparam_array::BParamArray, chunkdata::ChunkData, group_list::GroupList,
        voice_groups::VoiceGroups, zone_list::ZoneList,
    },
    read_bytes::ReadBytesExt,
    Error,
};

use super::program_data::{ProgramDataV80, ProgramDataVA5};

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum BProgram {
    ProgramDataV80(ProgramDataV80),
    ProgramDataVA5(ProgramDataVA5),
}

impl BProgram {
    /// BProgram::doReadPubPars
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<(), Error> {
        println!("BProgram::read");
        let version = reader.read_u16_le()?;

        match version {
            0x80 | 0x82 | 0x90 => {
                let len = reader.read_u32_le()? as usize;
                let _data = reader.read_bytes(len)?;
                // PrivParsV80::read(data.as_slice())?;

                // pubdata
                let len = reader.read_u32_le()? as usize;
                let data = reader.read_bytes(len)?;
                println!("{:?}", ProgramDataV80::read(data.as_slice())?);

                // children
                let len = reader.read_u32_le()? as usize;
                let data = reader.read_bytes(len)?;
                let mut data = data.as_slice();

                while let Ok(chunk) = ChunkData::read(&mut data) {
                    println!("child 0x{:x}", chunk.id);
                    match chunk.id {
                        0x32 => {
                            VoiceGroups::read(&mut chunk.data.as_slice())?;
                        }
                        0x33 => {
                            GroupList::read(&mut chunk.data.as_slice())?;
                        }
                        0x34 => {
                            ZoneList::read(&mut chunk.data.as_slice())?;
                        }
                        0x3A => {
                            BParamArray::read(&mut chunk.data.as_slice(), 8)?;
                        }
                        _ => {
                            panic!("unsupported child 0x{:x}", chunk.id);
                        }
                    };
                }
            }
            0x91 | 0x92 | 0xA0 | 0xA1 | 0xA2 | 0xA3 | 0xA4 | 0xA5 => {
                ProgramDataVA5::read(&mut reader)?;
            }
            0xA6 => {}
            0xA7 => {}
            0xA8 | 0xA9 | 0xAA | 0xAB | 0xAC | 0xAD | 0xAE => {}
            0xAF => {}
            _ => {}
        }

        // if version > 0xA1 {
        //     ??
        // }

        Ok(())
    }
}

#[test]
fn test_bprogram() -> Result<(), Error> {
    let file = include_bytes!("tests/bprogram/4.2.2.4504/000");
    assert!(BProgram::read(file.as_slice()).is_ok());
    Ok(())
}
