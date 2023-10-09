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
    kontakt::{objects::program::Program, Chunk, KontaktError},
    read_bytes::ReadBytesExt,
    Error,
};

#[derive(Debug)]
pub struct Kon6 {
    pub chunks: Vec<Chunk>,
}

impl Kon6 {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let mut chunks = Vec::new();
        while let Ok(chunk) = Chunk::read(&mut reader) {
            chunks.push(chunk);
        }

        Ok(Self { chunks })
    }

    pub fn program(&self) -> Result<Program, Error> {
        self.chunks
            .get(0)
            .ok_or(Error::KontaktError(KontaktError::MissingChunk(
                "Program".into(),
            )))
            .and_then(Program::try_from)
    }
}
