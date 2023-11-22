use crate::{
    kontakt::{
        objects::{FileNameListPreK51, Program},
        KontaktChunks,
    },
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

#[derive(Debug)]
pub struct KontaktV42 {
    pub program: Program,
    pub filetable: FileNameListPreK51,
}

impl std::convert::TryFrom<KontaktChunks> for KontaktV42 {
    type Error = Error;

    fn try_from(chunks: KontaktChunks) -> Result<Self, Self::Error> {
        Ok(Self {
            program: chunks
                .first()
                .ok_or(NIFileError::Static("Could not find Program".into()))?
                .try_into()?,
            filetable: chunks
                .last()
                .ok_or(NIFileError::Static(
                    "Could not find FileNameListPreK51".into(),
                ))?
                .try_into()?,
        })
    }
}
