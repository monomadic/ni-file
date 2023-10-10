// Kon5 Schema
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

use std::io::Cursor;

use crate::{
    kontakt::{objects::program::Program, Chunk},
    nks::{error::NKSError, BPatchMetaInfoHeader},
    read_bytes::ReadBytesExt,
    Error,
};

#[derive(Debug)]
pub struct Kon5 {
    pub chunks: Vec<Chunk>,
    pub program: Program, // TODO: check version?
                          // pub meta_info: BPatchMetaInfoHeader,
}

impl Kon5 {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let program: Program = Chunk::read(&mut reader).and_then(|chunk| (&chunk).try_into())?;

        let mut chunks = Vec::new();
        while let Ok(chunk) = Chunk::read(&mut reader) {
            chunks.push(chunk);
        }

        Ok(Self { chunks, program })
    }

    /// Decompress internal patch data
    pub fn from_compressed(
        compressed_data: Vec<u8>,
        _decompressed_length: usize,
    ) -> Result<Vec<Chunk>, Error> {
        Ok(Self::from(
            lz77::decompress(&mut Cursor::new(compressed_data))
                // TODO: error should be KontaktError
                .map_err(|e| NKSError::Decompression(e.to_string()))?,
        )?)
    }

    /// Parse patch data into Chunks
    pub fn from(decompressed_data: Vec<u8>) -> Result<Vec<Chunk>, Error> {
        let mut objects = Vec::new();
        let mut decompressed_data = Cursor::new(decompressed_data);

        while let Ok(chunk) = Chunk::read(&mut decompressed_data) {
            objects.push(chunk);
        }

        Ok(objects)
    }
}
