// use std::io::Cursor;
//
// use crate::{
//     kontakt::{
//         bparam_array::BParamArray, chunkdata::ChunkData, group_list::GroupList,
//         voice_groups::VoiceGroups, zone_list::ZoneList,
//     },
//     read_bytes::ReadBytesExt,
//     Error,
// };
//
// use super::program_data::{ProgramDataV80, ProgramDataVA5};
//
// #[derive(Debug)]
// #[allow(non_camel_case_types)]
// pub enum BProgram {
//     ProgramDataV80(ProgramDataV80),
//     ProgramDataVA5(ProgramDataVA5),
// }
//
// impl BProgram {
//     /// BProgram::doReadPubPars
//     pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<(), Error> {
//         let _do_read = reader.read_bool()?;
//
//         let version = reader.read_u16_le()?;
//         match version {
//             0x80 | 0x82 | 0x90 => {
//                 let len = reader.read_u32_le()? as usize;
//                 let _data = reader.read_bytes(len)?;
//                 // PrivParsV80::read(data.as_slice())?;
//
//                 // pubdata
//                 let len = reader.read_u32_le()? as usize;
//                 let _data = reader.read_bytes(len)?;
//
//                 // children
//                 let len = reader.read_u32_le()? as usize;
//                 let data = reader.read_bytes(len)?;
//                 let mut data = std::io::Cursor::new(data.as_slice());
//
//                 while let Ok(chunk) = ChunkData::read(&mut data) {
//                     let data = Cursor::new(chunk.data);
//
//                     match chunk.id {
//                         0x32 => {
//                             VoiceGroups::read(data)?;
//                         }
//                         0x33 => {
//                             GroupList::read(data)?;
//                         }
//                         0x34 => {
//                             ZoneList::read(data)?;
//                         }
//                         0x3A => {
//                             BParamArray::read(data, 8)?;
//                         }
//                         _ => {
//                             panic!("unsupported child 0x{:x}", chunk.id);
//                         }
//                     };
//                 }
//             }
//             0x91 | 0x92 | 0xA0 | 0xA1 | 0xA2 | 0xA3 | 0xA4 | 0xA5 => {
//                 // private
//                 let len = reader.read_u32_le()? as usize;
//                 let _data = reader.read_bytes(len)?;
//
//                 // pubdata
//                 let len = reader.read_u32_le()? as usize;
//                 let data = std::io::Cursor::new(reader.read_bytes(len)?);
//
//                 ProgramDataVA5::read(data)?;
//             }
//             0xA6 => {}
//             0xA7 => {}
//             0xA8 | 0xA9 | 0xAA | 0xAB | 0xAC | 0xAD | 0xAE => {}
//             0xAF => {}
//             _ => panic!("Unsupported BProgram Version: 0x{:x}", version),
//         }
//
//         // if version > 0xA1 {
//         //     ??
//         // }
//
//         Ok(())
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use std::fs::File;
//
//     use crate::Error;
//
//     use super::*;
//
//     #[test]
//     fn test_bprogram() -> Result<(), Error> {
//         let file = File::open("tests/patchdata/KontaktV42/Program/4.2.2.4504/000")?;
//         assert!(BProgram::read(file).is_ok());
//         Ok(())
//     }
// }
