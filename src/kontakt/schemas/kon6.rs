// Kon6 Schema
//
// 0x28 Program
//  0x3a    BParameterArraySer<BParFX,8>
//  0x3a    BParameterArraySer<BParFX,8>
//  0x45x16 BInsertBus
//  0x06x5  BParScript
//  0x4e    QuickBrowseData
//  0x32    VoiceGroups
//  0x33    GroupList
//  0x34    ZoneList
// 0x47 SaveSettings
// 0x4B FNTableImpl

use crate::{
    kontakt::{
        objects::{FNTableImpl, Program},
        KontaktChunks,
    },
    Error,
};

#[derive(Debug)]
pub struct Kon6 {
    pub program: Program,
    pub filetable: FNTableImpl,
}

impl std::convert::TryFrom<KontaktChunks> for Kon6 {
    type Error = Error;

    fn try_from(chunks: KontaktChunks) -> Result<Self, Self::Error> {
        Ok(Self {
            program: chunks
                .first()
                .ok_or(Error::Static("Could not find Program".into()))?
                .try_into()?,
            filetable: chunks
                .last()
                .ok_or(Error::Static("Could not find FNTableImpl".into()))?
                .try_into()?,
        })
    }
}
