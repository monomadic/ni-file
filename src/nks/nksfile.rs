use std::io::{Cursor, Read};

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
            0xB36EE55E | 0x7FA89012 =>{},
            _ => panic!("Invalid BPatchMetaInfoHeader magic number: expected 0xB36EE55E | 0x7FA89012 got 0x{magic:x}")
        };

        let _header_length = reader.read_u32_le()?;
        let header = BPatchHeader::read_le(&mut reader)?;
        let mut compressed_data = Vec::new();

        reader.read_to_end(&mut compressed_data)?;

        Ok(Self {
            header,
            compressed_data,
        })

        // match magic {
        //     // NKSv2
        //     0x7FA89012 => {
        //         let zlib_length = reader.read_u32_le()?;
        //         let header = BPatchHeader::read_le(&mut reader)?;
        //
        //         return Ok(match header {
        //             BPatchHeader::BPatchHeaderV2(v2) => {
        //                 let mut compressed_data = Vec::new();
        //
        //                 reader.read_to_end(&mut compressed_data)?;
        //
        //                 Self {
        //                     header: BPatchHeader::BPatchHeaderV2(v2),
        //                     compressed_data,
        //                 }
        //             }
        //             BPatchHeader::BPatchHeaderV42(v42) => unimplemented!(),
        //             _ => unreachable!(),
        //         });
        //
        //         // Ok(Self {
        //         //     header,
        //         //     preset: match v2.app_signature.as_str() {
        //         //         "Kon2" => KontaktPreset::Kon2(Kon2::read(&mut reader)?),
        //         //         _ => unsupported!("app signature"),
        //         //     },
        //         // })
        //     }
        //     _ => Err(NKSError::InvalidMagicNumber(magic).into()),
        // }
    }

    /// Decompress internal preset data
    pub fn preset(&self) -> Result<KontaktPreset, NKSError> {
        match self.header {
            BPatchHeader::BPatchHeaderV1(_) => {
                // Decompress the V1 preset xml document.
                let mut decoder = ZlibDecoder::new(&self.compressed_data[..]);
                let mut decompressed_data = Vec::new();
                decoder.read_to_end(&mut decompressed_data)?;

                Ok(KontaktPreset::Kon1(Kon1(
                    String::from_utf8(decompressed_data).expect("convert xml"),
                )))
            }
            BPatchHeader::BPatchHeaderV2(_) => todo!(),
            BPatchHeader::BPatchHeaderV42(v42) => {
                match v42.app_signature.as_str() {
                    "Kon4" => {}
                }

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
                    chunks
                    meta_info: BPatchMetaInfoHeader::read(&mut reader)?,
                }))
            }
        }
    }

    // pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, NIFileError> {
    //     let header = NKSHeader::read_le(&mut reader)?;
    //
    //     Ok(match header {
    //         NKSHeader::BPatchHeaderV1(_) => Self {
    //             header,
    //             preset: NKSPreset::Kon1(Kon1::read(&mut reader)?),
    //         },
    //         NKSHeader::BPatchHeaderV2(v2) => Self {
    //             header,
    //             preset: match v2.app_signature.as_str() {
    //                 "Kon2" => NKSPreset::Kon2(Kon2::read(&mut reader)?),
    //                 _ => unsupported!("app signature"),
    //             },
    //         },
    //         NKSHeader::BPatchHeaderV42(_) => todo!(),
    //     })
    // }
}

#[derive(Debug)]
pub enum KontaktPreset {
    Kon1(Kon1),
    Kon2(Kon2),
    Kon4(Kon4),
    // etc
}

#[derive(Debug)]
pub struct Kon1(String);

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

// #[derive(Debug)]
// pub struct NKSFile {
//     pub header: NKSHeader,
//     pub compressed_patch_data: Vec<u8>,
//     pub meta_info: Option<BPatchMetaInfoHeader>,
// }

// impl KontaktPreset {
//     pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, NKSError> {}
// }

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

// impl NKSFile {
//     pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, NIFileError> {
//         let magic = reader.read_u32_le()?;
//
//         match magic {
//             0xB36EE55E => {
//                 // NKSv1
//                 return Ok(NKSFile::Kon1(Kon1::read(&mut reader)?));
//             }
//             0x7FA89012 => {
//                 // NKSv2
//             }
//             _ => {
//                 return Err(NKSError::InvalidMagicNumber(magic).into());
//             }
//         }
//
//         let zlib_length = reader.read_u32_le()? as usize;
//         let header = NKSHeader::read_le(&mut reader)?;
//
//         match &header {
//             NKSHeader::BPatchHeaderV2(_) => {
//                 let mut compressed_data = Vec::new();
//                 reader.read_to_end(&mut compressed_data)?;
//
//                 let mut d = ZlibDecoder::new(&compressed_data[..]);
//                 let mut decompressed_data = Vec::new();
//                 d.read_to_end(&mut decompressed_data)?;
//
//                 Ok(NKSFile {
//                     header,
//                     compressed_patch_data: decompressed_data,
//                     meta_info: None,
//                 })
//             }
//
//             NKSHeader::BPatchHeaderV42(h) => {
//                 // deflate InternalPatchData
//                 let data = crate::deflate::deflate_with_lib(
//                     reader.read_bytes(zlib_length)?.as_slice(),
//                     h.decompressed_length as usize,
//                 )?;
//
//                 Ok(NKSFile {
//                     header,
//                     compressed_patch_data: data,
//                     meta_info: Some(BPatchMetaInfoHeader::read(&mut reader)?),
//                 })
//             }
//         }
//     }
//
//     pub fn decompress_patch_chunks(&self) -> Result<Vec<ChunkData>, Error> {
//         let mut objects = Vec::new();
//         let mut compressed_data = Cursor::new(&self.compressed_patch_data);
//
//         while let Ok(chunk) = ChunkData::read(&mut compressed_data) {
//             objects.push(chunk);
//         }
//
//         Ok(objects)
//     }
//
//     pub fn instrument(&self) -> Result<KontaktInstrument, Error> {
//         Ok(KontaktInstrument(self.decompress_patch_chunks()?))
//     }
// }

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn test_nksfile_read_v2() -> Result<(), NKSError> {
        let file = File::open("tests/filetype/NKS/KontaktV2/KontaktV2-000.nki")?;
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
