use std::{
    fmt::Display,
    io::{Cursor, Read},
};

use flate2::read::ZlibDecoder;

use crate::{
    kontakt::chunkdata::ChunkData, nks::meta_info::BPatchMetaInfoHeader, read_bytes::ReadBytesExt,
    Error,
};

use super::{error::NKSError, header::BPatchHeader};

#[derive(Debug)]
pub struct NKSContainer {
    pub header: BPatchHeader,
    pub compressed_data: Vec<u8>,
}

impl NKSContainer {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, NKSError> {
        let magic = reader.read_u32_le()?;

        match magic {
            0xB36EE55E | 0x7FA89012 | 0xA4D6E55A =>{
            },
            _ => panic!("Invalid BPatchMetaInfoHeader magic number: expected 0xB36EE55E | 0x7FA89012 | 0xA4D6E55A got 0x{magic:x}")
        };

        let _header_length = reader.read_u32_le()?;
        let header = BPatchHeader::read_le(&mut reader)?;
        let mut compressed_data = Vec::new();

        reader.read_to_end(&mut compressed_data)?;
        dbg!(&header);

        Ok(Self {
            header,
            compressed_data,
        })
    }

    /// Decompress internal preset data
    pub fn preset(&self) -> Result<KontaktPreset, NKSError> {
        match &self.header {
            BPatchHeader::BPatchHeaderV1(_) => {
                // V1 headers are always Kon1 files.
                let data = self.compressed_data.as_slice();
                Ok(KontaktPreset::Kon1(XMLDocument::from_compressed_data(
                    data,
                )?))
            }
            BPatchHeader::BPatchHeaderV2(v2) => match v2.app_signature.as_str() {
                "Kon3" => unimplemented!(),
                _ => unimplemented!(),
            },
            BPatchHeader::BPatchHeaderV42(v42) => {
                match v42.app_signature.as_str() {
                    "Kon4" => {
                        // Decompress the V1 preset xml document.
                        let mut decoder = ZlibDecoder::new(&self.compressed_data[..]);
                        let mut decompressed_data = Vec::new();
                        decoder.read_to_end(&mut decompressed_data)?;
                        let mut decompressed_data = Cursor::new(decompressed_data);

                        let mut chunks = Vec::new();
                        while let Ok(chunk) = ChunkData::read(&mut decompressed_data) {
                            chunks.push(chunk);
                        }

                        Ok(KontaktPreset::Kon4(Kon4 {
                            chunks,
                            meta_info: BPatchMetaInfoHeader::read(&mut decompressed_data)?,
                        }))
                    }
                    _ => unimplemented!(),
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum KontaktPreset {
    Kon1(XMLDocument),
    Kon2(XMLDocument),
    Kon3(XMLDocument),
    Kon4(Kon4),
    // etc
}

#[derive(Debug)]
pub struct XMLDocument(String);

impl XMLDocument {
    pub fn from_compressed_data(data: &[u8]) -> Result<Self, NKSError> {
        let mut decoder = ZlibDecoder::new(data);
        let mut decompressed_data = Vec::new();
        decoder.read_to_end(&mut decompressed_data)?;

        Ok(XMLDocument(
            String::from_utf8(decompressed_data).expect("convert xml"),
        ))
    }
}

impl Display for XMLDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

#[derive(Debug)]
pub struct Kon2 {
    pub zlib_length: u32,
    pub decompressed_length: u32,
    pub compressed_data: Vec<u8>,
    pub meta_info: BPatchMetaInfoHeader,
}

#[derive(Debug)]
pub struct Kon4 {
    pub chunks: Vec<ChunkData>,
    pub meta_info: BPatchMetaInfoHeader,
}

impl Kon4 {
    /// Decompress internal patch data
    pub fn from_compressed(
        compressed_data: Vec<u8>,
        decompressed_length: usize,
    ) -> Result<Vec<ChunkData>, Error> {
        Ok(Self::from(crate::deflate::deflate_with_lib(
            &compressed_data,
            decompressed_length,
        )?)?)
    }

    /// Parse patch data into Chunks
    pub fn from(decompressed_data: Vec<u8>) -> Result<Vec<ChunkData>, Error> {
        let mut objects = Vec::new();
        let mut decompressed_data = Cursor::new(decompressed_data);

        while let Ok(chunk) = ChunkData::read(&mut decompressed_data) {
            objects.push(chunk);
        }

        Ok(objects)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn test_nksv1_nki_0x5ee56eb3() -> Result<(), NKSError> {
        let file = File::open("tests/filetype/NKS/KontaktV1/000-NKSv1-NKI.nki")?;
        let nks = NKSContainer::read(file)?;

        assert!(matches!(nks.header, BPatchHeader::BPatchHeaderV1(_)));
        Ok(())
    }

    #[test]
    fn test_nksfile_read_v2_single() -> Result<(), NKSError> {
        let file = File::open("tests/filetype/NKS/KontaktV2/KontaktV2-000-empty.nki")?;
        let _nks = NKSContainer::read(file)?;
        Ok(())
    }

    #[test]
    fn test_nksfile_read_v42() -> Result<(), NKSError> {
        let file = File::open("tests/filetype/NKS/KontaktV42/4.2.4.5316-000.nki")?;
        let _nks = NKSContainer::read(file)?;
        Ok(())
    }
}
