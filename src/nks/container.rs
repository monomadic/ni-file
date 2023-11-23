use std::io::{Cursor, Read};

use flate2::read::ZlibDecoder;

use crate::{
    kontakt::{
        objects::{BPatchHeader, BPatchHeaderV42, BPatchMetaInfoHeader},
        schemas::{KontaktPreset, KontaktV1, KontaktV2},
        KontaktPatch,
    },
    read_bytes::ReadBytesExt,
    Error,
};

use super::error::NKSError;

#[derive(Debug)]
pub struct NKSContainer {
    pub header: BPatchHeader,
    pub compressed_data: Vec<u8>,
    pub meta_info: Option<BPatchMetaInfoHeader>,
}

impl NKSContainer {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, NKSError> {
        let magic = reader.read_u32_le()?;

        // NOTE: 0xab85ef01 is also valid
        match magic {
            0xB36EE55E | 0x7FA89012 | 0xA4D6E55A | 0x10874353 =>{
            },
            _ => panic!("Invalid NKSContainer magic number: expected 0xB36EE55E | 0x7FA89012 | 0xA4D6E55A got 0x{magic:x}")
        };

        // For BPatchHeaderV1, this field is zlib_start
        let compressed_length = reader.read_u32_le()? as usize;
        let header = BPatchHeader::read_le(&mut reader)?;

        let compressed_data = match header {
            BPatchHeader::BPatchHeaderV1(_) => reader.read_all()?,
            BPatchHeader::BPatchHeaderV2(ref h) => match h.is_monolith {
                true => unimplemented!("monolith"),
                false => {
                    if compressed_length == 0 {
                        let mut buf = Vec::new();
                        reader.read_to_end(&mut buf)?;
                        buf
                    } else {
                        reader.read_bytes(compressed_length)?
                    }
                }
            },
            BPatchHeader::BPatchHeaderV42(ref h) => match h.is_monolith {
                true => unimplemented!("monolith"),
                false => reader.read_bytes(compressed_length)?,
            },
        };

        // std::fs::write("compressed", &compressed_data)?;

        let footer_raw = reader.read_all()?;
        let meta_info = match header {
            BPatchHeader::BPatchHeaderV1(_) => None,
            BPatchHeader::BPatchHeaderV2(_) => None,
            BPatchHeader::BPatchHeaderV42(_) => {
                Some(BPatchMetaInfoHeader::read(&mut Cursor::new(footer_raw))?)
            }
        };

        // let meta_info = None;

        // std::fs::write("compressed", &reader.read_all()?)?;

        Ok(Self {
            header,
            compressed_data,
            meta_info,
        })
    }

    /// Decompress raw internal preset data
    pub fn decompressed_preset(&self) -> Result<Vec<u8>, Error> {
        assert!(self.compressed_data.len() > 0, "no compressed data");
        let reader = Cursor::new(&self.compressed_data);

        Ok(match &self.header {
            BPatchHeader::BPatchHeaderV1(_) => {
                // zlib compression
                let mut decoder = ZlibDecoder::new(reader);
                let mut decompressed_data = Vec::new();
                decoder.read_to_end(&mut decompressed_data)?;

                decompressed_data
            }
            BPatchHeader::BPatchHeaderV2(_) => {
                // zlib compression
                let mut decoder = ZlibDecoder::new(reader);
                let mut decompressed_data = Vec::new();
                decoder.read_to_end(&mut decompressed_data)?;

                decompressed_data
            }
            BPatchHeader::BPatchHeaderV42(ref h) => {
                // fastlz decompression
                // let decompressed_data = lz77::decompress(reader).expect("lz77");

                let decompressed_size = h.decompressed_length as usize;

                let decompressed_data = &mut vec![0_u8; decompressed_size];
                fastlz::decompress(&self.compressed_data, decompressed_data)
                    .map_err(|_| Error::Generic("fastlz".into()))?
                    .to_vec();

                assert_eq!(h.decompressed_length as usize, decompressed_data.len());

                decompressed_data.to_vec()
            }
        })
    }

    /// Decompress internal preset data and return a KontaktPreset
    pub fn preset(&self) -> Result<KontaktPreset, Error> {
        assert!(self.compressed_data.len() > 0, "No compressed data");

        let reader = Cursor::new(&self.compressed_data);

        Ok(match &self.header {
            BPatchHeader::BPatchHeaderV1(_) => {
                // zlib compression
                let mut decoder = ZlibDecoder::new(reader);
                let mut decompressed_data = Vec::new();
                decoder.read_to_end(&mut decompressed_data)?;
                let mut raw_preset = Cursor::new(decompressed_data);

                KontaktPreset::KontaktV1(KontaktV1::read(&mut raw_preset)?)
            }
            BPatchHeader::BPatchHeaderV2(_) => {
                // zlib compression
                let mut decoder = ZlibDecoder::new(reader);
                let mut decompressed_data = Vec::new();
                decoder.read_to_end(&mut decompressed_data)?;
                let raw_preset = Cursor::new(decompressed_data);

                KontaktPreset::KontaktV2(KontaktV2::read(raw_preset)?)
            }
            BPatchHeader::BPatchHeaderV42(header) => {
                // fastlz compression
                let data = self.decompressed_preset()?;
                let header: BPatchHeaderV42 = header.clone(); // an ugly clone

                KontaktPatch { header, data }.preset()?
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn test_nksv1_nki_0x5ee56eb3() -> Result<(), NKSError> {
        let file = File::open("tests/data/Containers/NKS/KontaktV1/000-kontaktv1-nki.nki")?;
        let nks = NKSContainer::read(file)?;

        assert!(matches!(nks.header, BPatchHeader::BPatchHeaderV1(_)));
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_nksfile_read_phv2_monolith_kon2_nki() -> Result<(), NKSError> {
        let file =
            File::open("tests/data/Containers/NKS/KontaktV2/000-phv2_monolith_kon2_nki.nki")?;
        let _nks = NKSContainer::read(file)?;
        Ok(())
    }

    #[test]
    fn test_nksfile_read_v42() -> Result<(), NKSError> {
        let file = File::open("tests/data/Containers/NKS/KontaktV42/4.2.4.5316-000.nki")?;
        let nks = NKSContainer::read(file)?;

        dbg!(nks.meta_info);
        // let _preset = nks.preset().unwrap();
        Ok(())
    }
}
