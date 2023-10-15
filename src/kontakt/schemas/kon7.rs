// Kon7 Schema
//
// 0x28 Program
//  0x3a    BParameterArraySer<BParFX,8>
//  0x3a    BParameterArraySer<BParFX,8>
//  0x45x16 BInsertBus
//  0x06x5  BParScript
//  0x4e    QuickBrowseData
//  0x3a    BParameterArraySer<BParFX,8>
//  0x32    VoiceGroups
//  0x33    GroupList
//  0x34    ZoneList
// 0x47 SaveSettings
// 0x4B FNTableImpl

use crate::{
    kontakt::{
        chunk_set::KontaktChunks,
        objects::{FNTableImpl, Program},
        Chunk,
    },
    read_bytes::ReadBytesExt,
    Error,
};

#[derive(Debug)]
pub struct Kon7 {
    pub program: Program,
    pub filetable: FNTableImpl,
}

impl Kon7 {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let program: Program = Chunk::read(&mut reader).and_then(|chunk| (&chunk).try_into())?;

        // SaveSettings
        let _ = Chunk::read(&mut reader)?;

        let filetable: FNTableImpl =
            Chunk::read(&mut reader).and_then(|chunk| (&chunk).try_into())?;

        Ok(Self { program, filetable })
    }
}

impl std::convert::TryFrom<KontaktChunks> for Kon7 {
    type Error = Error;

    fn try_from(chunks: KontaktChunks) -> Result<Self, Self::Error> {
        Ok(Self {
            program: (&chunks.0[0]).try_into()?,
            filetable: (&chunks.0[2]).try_into()?,
        })
    }
}
