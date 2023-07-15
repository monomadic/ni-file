use crate::{read_bytes::ReadBytesExt, Error};

use super::{chunkdata::ChunkData, filename_list::FileNameListPreK51, objects::program::BProgram};

#[derive(Debug)]
pub struct InternalPatchData;

impl InternalPatchData {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        while let Ok(chunk) = ChunkData::read(&mut reader) {
            let mut reader = chunk.data.as_slice();

            match chunk.id {
                0x28 => {
                    BProgram::read(&mut reader)?;
                }
                0x3D => {
                    FileNameListPreK51::read(&mut reader)?;
                }
                _ => panic!("Unknown Object: 0x{:x}", chunk.id),
            };
        }

        Ok(Self {})
    }
}

#[test]
fn test_internal_patch_data() -> Result<(), Error> {
    let file = include_bytes!("tests/internal_patch_data/4.2.2.4504/000");
    InternalPatchData::read(file.as_slice())?;

    // let file = include_bytes!("tests/internal_patch_data/5.3.0.6464/000");
    // InternalPatchData::read(file.as_slice())?;
    Ok(())
}
