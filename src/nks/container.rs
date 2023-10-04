use std::io::Cursor;

use crate::{kontakt::KontaktPreset, read_bytes::ReadBytesExt};

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

        Ok(Self {
            header,
            compressed_data,
        })
    }

    /// Decompress internal preset data
    pub fn preset(&self) -> Result<KontaktPreset, NKSError> {
        let mut reader = Cursor::new(&self.compressed_data);

        match &self.header {
            BPatchHeader::BPatchHeaderV1(_) => {
                Ok(KontaktPreset::from_str(&mut reader, "Kon1").unwrap())
            }
            BPatchHeader::BPatchHeaderV2(v2) => {
                Ok(KontaktPreset::from_str(&mut reader, v2.app_signature.as_str()).unwrap())
            }
            BPatchHeader::BPatchHeaderV42(v42) => {
                Ok(KontaktPreset::from_str(&mut reader, v42.app_signature.as_str()).unwrap())
            }
        }
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
