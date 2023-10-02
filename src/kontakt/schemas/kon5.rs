// Kon5 Schema:
//
// 0x28 Program
//  0x3A BParameterArraySer<BParFX,8>
//  0x3A BParameterArraySer<BParFX,8>
//  0x45 x ? BInsertBus
//  ...
// 0x47 SaveSettings
// 0x4B FNTableImpl

use std::io::Cursor;

use crate::{
    kontakt::Chunk,
    nks::{error::NKSError, BPatchMetaInfoHeader},
    Error,
};

#[derive(Debug)]
pub struct Kon4 {
    pub chunks: Vec<Chunk>,
    pub meta_info: BPatchMetaInfoHeader,
}

impl Kon4 {
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
