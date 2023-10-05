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

use crate::{kontakt::Chunk, read_bytes::ReadBytesExt, Error};

#[derive(Debug)]
pub struct Kon7 {
    pub chunks: Vec<Chunk>,
}

impl Kon7 {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let mut chunks = Vec::new();
        while let Ok(chunk) = Chunk::read(&mut reader) {
            chunks.push(chunk);
        }

        Ok(Self { chunks })
    }

    // pub fn read_monolith<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {}

    //     pub fn program(&self) -> Result<Program, KontaktError> {}
    //     pub fn filenamelist(&self) -> Result<FNTableImpl, KontaktError> {}
}
