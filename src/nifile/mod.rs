// mod kontakt;
//
// use crate::{nks::nksfile::NKSFile, read_bytes::*, Error, NIFileType};
//
// use self::kontakt::KontaktInstrument;
//
// pub enum NIFile {
//     KontaktInstrument(KontaktInstrument),
// }
//
// impl NIFile {
//     pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
//         let cursor = reader.by_ref();
//         match NIFileType::detect(cursor)? {
//             NIFileType::NISound => todo!(),
//             NIFileType::FileContainer => todo!(),
//             NIFileType::NICompressedWave => todo!(),
//             NIFileType::KoreSound => todo!(),
//             NIFileType::Kontakt1 => todo!(),
//             NIFileType::NKS => {
//                 let nks = NKSFile::read(&mut reader)?;
//                 let chunks = nks.decompress_patch_chunks()?;
//                 Ok(NIFile::KontaktInstrument(KontaktInstrument(chunks)))
//             }
//             NIFileType::KontaktResource => todo!(),
//             NIFileType::KontaktCache => todo!(),
//             NIFileType::Unknown => todo!(),
//         }
//     }
// }
