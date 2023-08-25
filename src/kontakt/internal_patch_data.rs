// TODO: unneeded module?

// use std::io::Cursor;
//
// use super::{chunkdata::ChunkData, structured_object::StructuredObject};
// use crate::{read_bytes::ReadBytesExt, Error};
//
// #[derive(Debug)]
// pub struct InternalPatchData;
//
// impl InternalPatchData {
//     pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
//         while let Ok(chunk) = ChunkData::read(&mut reader) {
//             StructuredObject::read(Cursor::new(chunk.data))?.data()?;
//         }
//         Ok(Self {})
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_internal_patch_data() -> Result<(), Error> {
//         let file = std::io::Cursor::new(include_bytes!(
//             "../../tests/patchdata/KontaktV42/internal_patch_data/4.2.2.4504/000"
//         ));
//         InternalPatchData::read(file)?;
//
//         // let file = include_bytes!("tests/internal_patch_data/5.3.0.6464/000");
//         // InternalPatchData::read(file.as_slice())?;
//         Ok(())
//     }
// }
