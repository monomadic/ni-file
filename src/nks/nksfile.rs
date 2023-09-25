use std::io::{Cursor, Read};

use flate2::read::ZlibDecoder;

use crate::{
    kontakt::{chunkdata::ChunkData, instrument::KontaktInstrument},
    nks::meta_info::BPatchMetaInfoHeader,
    read_bytes::ReadBytesExt,
    Error, NIFileError,
};

use super::{
    error::NKSError,
    header::{BPatchHeaderV1, BPatchHeaderV2, BPatchHeaderV42, NKSHeader},
};

#[derive(Debug)]
pub enum NKSContainer {
    V1(NKSv1),
    V2(NKSv2),
    V42(NKSv42),
}

#[derive(Debug)]
pub struct NKSv1 {
    pub header: BPatchHeaderV1,
    pub compressed_data: Vec<u8>,
}

#[derive(Debug)]
pub struct NKSv2 {
    pub header: BPatchHeaderV2,
    pub zlib_length: u32,
    pub compressed_data: Vec<u8>,
    pub meta_info: BPatchMetaInfoHeader,
}

#[derive(Debug)]
pub struct NKSv42 {
    pub header: BPatchHeaderV42,
    pub zlib_length: u32,
    pub compressed_data: Vec<u8>,
    pub meta_info: BPatchMetaInfoHeader,
}

#[derive(Debug)]
pub struct NKSFile {
    pub header: NKSHeader,
    pub compressed_patch_data: Vec<u8>,
    pub meta_info: Option<BPatchMetaInfoHeader>,
}

impl NKSv1 {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, NIFileError> {
        let header = BPatchHeaderV1::read_le(&mut reader)?;

        let mut compressed_data = Vec::new();
        reader.read_to_end(&mut compressed_data)?;

        Ok(NKSv1 {
            header,
            compressed_data,
        })
    }

    /// Decompress the V1 preset xml document.
    pub fn preset_xml(&self) -> Result<String, NIFileError> {
        // note: decompress with zlib-flate -uncompress < in > out (from the qpdf package)
        let mut decoder = ZlibDecoder::new(&self.compressed_data[..]);
        let mut decompressed_data = Vec::new();
        decoder.read_to_end(&mut decompressed_data)?;

        // TODO: error
        Ok(String::from_utf8(decompressed_data).unwrap())
    }
}

impl NKSv2 {
    pub fn read<R: ReadBytesExt>(mut reader: R, zlib_length: u32) -> Result<Self, NIFileError> {
        // FIXME: doesn't work with multis

        Ok(NKSv2 {
            header: BPatchHeaderV2::read_le(&mut reader)?,
            zlib_length,
            compressed_data: reader.read_bytes(zlib_length as usize)?,
            meta_info: BPatchMetaInfoHeader::read(&mut reader)?,
        })
    }
}

impl NKSv42 {
    pub fn read<R: ReadBytesExt>(mut reader: R, zlib_length: u32) -> Result<Self, NIFileError> {
        Ok(NKSv42 {
            header: BPatchHeaderV42::read_le(&mut reader)?,
            zlib_length,
            compressed_data: reader.read_bytes(zlib_length as usize)?,
            meta_info: BPatchMetaInfoHeader::read(&mut reader)?,
        })
    }

    /// Decompress internal patch data
    pub fn decompress_patch_data(&self) -> Result<Vec<u8>, Error> {
        crate::deflate::deflate_with_lib(
            &self.compressed_data,
            self.header.decompressed_length as usize,
        )
    }

    /// Parse patch data into Chunks
    pub fn decompress_patch_chunks(&self) -> Result<Vec<ChunkData>, Error> {
        let mut objects = Vec::new();
        let mut decompressed_data = Cursor::new(self.decompress_patch_data()?);

        while let Ok(chunk) = ChunkData::read(&mut decompressed_data) {
            objects.push(chunk);
        }

        Ok(objects)
    }
}

impl NKSContainer {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, NIFileError> {
        let magic = reader.read_u32_le()?;

        match magic {
            0xB36EE55E => Ok(Self::V1(NKSv1::read(&mut reader)?)),
            0x7FA89012 => {
                let zlib_length = reader.read_u32_le()?;
                let header_version = reader.read_u16_le()?;

                match header_version {
                    0x0100 => Ok(Self::V2(NKSv2::read(&mut reader, zlib_length)?)),
                    0x0110 => Ok(Self::V42(NKSv42::read(&mut reader, zlib_length)?)), // BPatchHeaderV42::read_le(&mut reader)?),
                    _ => panic!("Unsupported header version: 0x{:x}", header_version),
                }
            }
            _ => Err(NKSError::InvalidMagicNumber(magic).into()),
        }
    }
}

impl NKSFile {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, NIFileError> {
        let magic = reader.read_u32_le()?;

        match magic {
            0xB36EE55E => {
                // v1
            }
            0x7FA89012 => {}
            _ => {
                return Err(NKSError::InvalidMagicNumber(magic).into());
            }
        }

        let zlib_length = reader.read_u32_le()? as usize;
        let header = NKSHeader::read_le(&mut reader)?;

        match &header {
            NKSHeader::BPatchHeaderV2(_) => {
                let mut compressed_data = Vec::new();
                reader.read_to_end(&mut compressed_data)?;

                // note: decompress with zlib-flate -uncompress < in > out
                //          (from the qpdf package)

                let mut d = ZlibDecoder::new(&compressed_data[..]);
                let mut decompressed_data = Vec::new();
                d.read_to_end(&mut decompressed_data)?;

                Ok(NKSFile {
                    header,
                    compressed_patch_data: decompressed_data,
                    meta_info: None,
                })
            }

            NKSHeader::BPatchHeaderV42(h) => {
                // deflate InternalPatchData
                let data = crate::deflate::deflate_with_lib(
                    reader.read_bytes(zlib_length)?.as_slice(),
                    h.decompressed_length as usize,
                )?;

                Ok(NKSFile {
                    header,
                    compressed_patch_data: data,
                    meta_info: Some(BPatchMetaInfoHeader::read(&mut reader)?),
                })
            }
        }
    }

    pub fn decompress_patch_chunks(&self) -> Result<Vec<ChunkData>, Error> {
        let mut objects = Vec::new();
        let mut compressed_data = Cursor::new(&self.compressed_patch_data);

        while let Ok(chunk) = ChunkData::read(&mut compressed_data) {
            objects.push(chunk);
        }

        Ok(objects)
    }

    pub fn instrument(&self) -> Result<KontaktInstrument, Error> {
        Ok(KontaktInstrument(self.decompress_patch_chunks()?))
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn test_nksfile_read_v42() -> Result<(), NIFileError> {
        let file = File::open("tests/filetype/NKS/KontaktV42/4.2.4.5316-000.nki")?;
        let _nks = NKSFile::read(file)?;
        Ok(())
    }
}
