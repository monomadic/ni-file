use std::io::{Cursor, Read};

use flate2::read::ZlibDecoder;

use crate::{read_bytes::ReadBytesExt, Error};

// Kon4 Schema:
//
// 0x28 Program
//  0x3A BParameterArray<BParFX,8>
//  0x3A BParameterArray<BParFX,8>
//  0x32 VoiceGroups
//  0x33 GroupList
//  0x34 ZoneList
// 0x3d FileNameListPreK1

use crate::{kontakt::Chunk, nks::BPatchMetaInfoHeader};

#[derive(Debug)]
pub struct Kon4 {
    pub chunks: Vec<Chunk>,
    pub meta_info: BPatchMetaInfoHeader,
}

impl Kon4 {
    pub fn read<R: ReadBytesExt>(reader: &mut R) -> Result<Self, Error> {
        // Decompress the V1 preset xml document.
        let mut decoder = ZlibDecoder::new(reader);
        let mut decompressed_data = Vec::new();
        decoder.read_to_end(&mut decompressed_data)?;
        let mut decompressed_data = Cursor::new(decompressed_data);

        let mut chunks = Vec::new();
        while let Ok(chunk) = Chunk::read(&mut decompressed_data) {
            chunks.push(chunk);
        }

        Ok(Self {
            chunks,
            meta_info: BPatchMetaInfoHeader::read(&mut decompressed_data)?,
        })
    }

    // pub fn read_monolith<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {}

    //     pub fn program(&self) -> Result<Program, KontaktError> {}
    //     pub fn filenamelist(&self) -> Result<FNTableImpl, KontaktError> {}
}
