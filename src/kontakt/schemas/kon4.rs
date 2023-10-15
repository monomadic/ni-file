use crate::{
    kontakt::objects::{FileNameListPreK51, Program},
    read_bytes::ReadBytesExt,
    Error, NIFileError,
};

// Kon4 Schema:
//
// 0x28 Program
//  0x3A BParameterArray<BParFX,8>
//  0x3A BParameterArray<BParFX,8>
//  0x32 VoiceGroups
//  0x33 GroupList
//  0x34 ZoneList
// 0x3d FileNameListPreK1

use crate::kontakt::Chunk;

#[derive(Debug)]
pub struct KontaktV42 {
    pub program: Program,
    pub filetable: FileNameListPreK51,
}

impl KontaktV42 {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let program: Program = Chunk::read(&mut reader).and_then(|chunk| (&chunk).try_into())?;
        let filetable: FileNameListPreK51 = Chunk::read(&mut reader)
            .and_then(|chunk| (&chunk).try_into())
            .map_err(|_| NIFileError::Generic("FileNameListPreK51".into()))?;

        Ok(Self { program, filetable })
    }
}
