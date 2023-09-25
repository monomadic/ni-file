///
/// magic = 0xa4d6e55a || 0xab85ef01 || 0xb36ee55e || 0x10874353 ||  0x74b5a69b || 0x7fa89012
///
/// headerVersion
///     < 256           36 bytes
///     > 256 && < 271  170 bytes
///     > 271           222 bytes
///
use std::io::Cursor;

use time::OffsetDateTime;

use crate::{read_bytes::ReadBytesExt, NIFileError};

/// The header of a Kontakt42 NKS File.
#[derive(Debug)]
pub struct BPatchHeaderV42 {
    pub patch_type: PatchType,
    pub app_version: NKIAppVersion,
    pub icon: u32,
    pub author: String,
    pub created_at: time::Date,
    pub app_signature: String,
    pub number_of_zones: u16,
    pub number_of_groups: u16,
    pub number_of_instruments: u16,
    pub decompressed_length: u32,
}

/// The header of a Kontakt2 NKS File.
#[derive(Debug)]
pub struct BPatchHeaderV2 {
    pub patch_type: PatchType,
    pub app_version: NKIAppVersion,
    pub icon: u32,
    pub author: String,
    pub created_at: time::Date,
    pub app_signature: String,
    pub number_of_zones: u16,
    pub number_of_groups: u16,
    pub number_of_instruments: u16,
    pub decompressed_length: u32,
}

/// The header of a Kontakt1 NKS File.
#[derive(Debug)]
pub struct BPatchHeaderV1 {
    pub created_at: time::Date,
    pub samples_size: u32,
    pub checksum: u64,
}

impl BPatchHeaderV1 {
    pub fn read_le<R: ReadBytesExt>(mut reader: R) -> Result<Self, NIFileError> {
        let _header_length = reader.read_u32_le()?;

        dbg!(reader.read_u16_le()?); // unknown
        dbg!(reader.read_u16_le()?); // version? usually 2
        dbg!(reader.read_u32_le()?); // ?
        dbg!(reader.read_u32_le()?); // ?
        dbg!(reader.read_u32_le()?); // ?

        let timestamp = OffsetDateTime::from_unix_timestamp(reader.read_u32_le()? as i64).unwrap();
        let created_at: time::Date = timestamp.date();
        let samples_size = reader.read_u32_le()?; // total size of all samples

        dbg!(reader.read_u32_le()?); // always 0

        let checksum = reader.read_u64_le()?;

        Ok(Self {
            created_at,
            samples_size,
            checksum,
        })
    }
}

impl BPatchHeaderV2 {
    pub fn read_le<R: ReadBytesExt>(mut reader: R) -> Result<Self, NIFileError> {
        let header_magic = reader.read_u32_le()?;
        assert_eq!(header_magic, u32::swap_bytes(0x722A013E));

        let patch_type: PatchType = reader.read_u16_le()?.into();
        let app_version = NKIAppVersion {
            minor_3: reader.read_u8()?,
            minor_2: reader.read_u8()?,
            minor_1: reader.read_u8()?,
            major: reader.read_u8()?,
        };

        let mut app_signature = reader.read_bytes(4)?;
        app_signature.reverse();
        let app_signature: String = app_signature.into_iter().map(|c| c as char).collect();

        let datetime = OffsetDateTime::from_unix_timestamp(reader.read_u32_le()? as i64).unwrap();
        let created_at: time::Date = datetime.date();
        let _unknown = reader.read_u32_le()?;
        let number_of_zones = reader.read_u16_le()?;
        let number_of_groups = reader.read_u16_le()?;
        let number_of_instruments = reader.read_u16_le()?;
        let _unknown = reader.read_bytes(16)?;
        let icon = reader.read_u32_le()?;
        let author = reader.read_string_utf8()?;
        // let _unknown = reader.read_bytes(101)?;
        let _checksum = reader.read_bytes(16)?;
        let _unknown = reader.read_u32_le()?;
        let _unknown = reader.read_u32_le()?;
        let decompressed_length = reader.read_u32_le()?;

        // seems all zero bytes
        //let _unknown = reader.read_bytes(32)?;

        Ok(Self {
            patch_type,
            app_version,
            icon,
            author,
            number_of_zones,
            number_of_groups,
            number_of_instruments,
            created_at,
            app_signature,
            decompressed_length,
        })
    }
}

impl BPatchHeaderV42 {
    pub fn read_le<R: ReadBytesExt>(mut reader: R) -> Result<Self, NIFileError> {
        let magic: u32 = reader.read_le()?;

        assert_eq!(
            magic, 0xEA37631A,
            "Invalid BPatchHeaderV42 magic number: expected 0x1a6337ea got 0x{magic:x}"
        );

        let patch_type: PatchType = reader.read_u16_le()?.into();
        let app_version = NKIAppVersion {
            minor_3: reader.read_u8()?,
            minor_2: reader.read_u8()?,
            minor_1: reader.read_u8()?,
            major: reader.read_u8()?,
        };

        let mut app_signature = reader.read_bytes(4)?;
        app_signature.reverse();
        let app_signature: String = app_signature.into_iter().map(|c| c as char).collect();

        let datetime = OffsetDateTime::from_unix_timestamp(reader.read_u32_le()? as i64).unwrap();
        let created_at: time::Date = datetime.date();
        let _unknown = reader.read_u32_le()?;
        let number_of_zones = reader.read_u16_le()?;
        let number_of_groups = reader.read_u16_le()?;
        let number_of_instruments = reader.read_u16_le()?;
        let _unknown = reader.read_bytes(16)?;
        let icon = reader.read_u32_le()?;

        let embedded_strings = reader.read_bytes(104)?;
        let mut strings = Cursor::new(embedded_strings);
        let author = strings.read_string_utf8()?;

        let _checksum = reader.read_bytes(16)?;
        let _unknown = reader.read_u32_le()?;
        let _unknown = reader.read_u32_le()?;
        let decompressed_length = reader.read_u32_le()?;

        // seems all zero bytes
        let _unknown = reader.read_bytes(32)?;

        Ok(Self {
            patch_type,
            app_version,
            icon,
            author,
            number_of_zones,
            number_of_groups,
            number_of_instruments,
            created_at,
            app_signature,
            decompressed_length,
        })
    }
}

pub struct NKIAppVersion {
    pub major: u8,
    pub minor_1: u8,
    pub minor_2: u8,
    pub minor_3: u8,
}

impl std::fmt::Debug for NKIAppVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}",
            self.major, self.minor_1, self.minor_2, self.minor_3
        )
    }
}

impl std::fmt::Display for NKIAppVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "{}.{}.{}.{}",
            self.major, self.minor_2, self.minor_2, self.minor_3,
        ))
    }
}

#[derive(Debug, PartialEq)]
pub enum PatchType {
    NKM,
    NKI,
    NKB,
    NKP,
    NKG,
    NKZ,
    Unknown(u16),
}

impl From<u16> for PatchType {
    fn from(value: u16) -> Self {
        use PatchType::*;
        match value {
            0 => NKM,
            1 => NKI,
            2 => NKB,
            3 => NKP,
            4 => NKG,
            5 => NKZ,
            _ => Unknown(value),
        }
    }
}

#[derive(Debug)]
pub enum NKSHeader {
    BPatchHeaderV2(BPatchHeaderV2),
    BPatchHeaderV42(BPatchHeaderV42),
}

impl NKSHeader {
    pub fn read_le<R: ReadBytesExt>(mut reader: R) -> Result<Self, NIFileError> {
        let header_version = reader.read_u16_le()?;
        Ok(match header_version {
            // Kontakt2
            0x0100 => NKSHeader::BPatchHeaderV2(BPatchHeaderV2::read_le(&mut reader)?),
            // Kontakt42
            0x0110 => NKSHeader::BPatchHeaderV42(BPatchHeaderV42::read_le(&mut reader)?),
            // Unknown
            _ => panic!("Unsupported header version: 0x{:x}", header_version),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn test_header_v1_read() -> Result<(), NIFileError> {
        let file = File::open("tests/patchdata/NKS/BPatchHeaderV1/BPatchHeaderV1-000")?;
        println!("{:?}", BPatchHeaderV1::read_le(file)?);
        Ok(())
    }

    #[test]
    fn test_header_v2_read() -> Result<(), NIFileError> {
        let file = File::open("tests/patchdata/NKS/BPatchHeaderV2/000")?;
        // let mut reader = file.as_slice();
        // NKSHeader::read_le(file.as_slice())?;
        println!("{:?}", NKSHeader::read_le(file)?);
        Ok(())
    }

    #[test]
    fn test_header_v42_read() -> Result<(), NIFileError> {
        let file = File::open("tests/patchdata/NKS/BPatchHeaderV42/000")?;
        println!("{:?}", NKSHeader::read_le(file)?);
        Ok(())
    }
}
