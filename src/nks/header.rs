use time::OffsetDateTime;

use crate::{read_bytes::ReadBytesExt, NIFileError};

// pub enum KonaktPatchHeader {
//     BPatchHeaderV42(BPatchHeaderV42),
// }
//
// impl KonaktPatchHeader {
//     pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {}
// }

/// The header of a Kontakt42 preset.
///
/// magic = 0xa4d6e55a || 0xab85ef01 || 0xb36ee55e || 0x10874353 ||  0x74b5a69b || 0x7fa89012
///
/// headerVersion
///     < 256           36 bytes
///     > 256 && < 271  170 bytes
///     > 271           222 bytes
///

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

impl BPatchHeaderV2 {
    pub fn read_le<R: ReadBytesExt>(mut reader: R) -> Result<Self, NIFileError> {
        let reader = reader.read_bytes(170 - 10)?;
        let mut reader = reader.as_slice();

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
        let reader = reader.read_bytes(222 - 10)?;
        let mut reader = reader.as_slice();

        let patch_version = reader.read_u32_le()?;
        assert_eq!(patch_version, u32::swap_bytes(0x1A6337EA));

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
        let mut strings = embedded_strings.as_slice();
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

#[derive(Debug)]
pub struct NKIAppVersion {
    pub major: u8,
    pub minor_1: u8,
    pub minor_2: u8,
    pub minor_3: u8,
}

impl ToString for NKIAppVersion {
    fn to_string(&self) -> String {
        format!(
            "app_version {}.{}.{}.{}",
            self.major, self.minor_2, self.minor_2, self.minor_3
        )
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
            // // Kontakt2 LE
            0x0100 => NKSHeader::BPatchHeaderV2(BPatchHeaderV2::read_le(&mut reader)?),
            // // Kontakt42 LE
            0x0110 => NKSHeader::BPatchHeaderV42(BPatchHeaderV42::read_le(&mut reader)?),
            // Unknown
            _ => panic!("Unsupported header version: 0x{:x}", header_version),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_v2_read() -> Result<(), NIFileError> {
        let file = include_bytes!("../../tests/patchdata/NKS/BPatchHeaderV2/000");
        // let mut reader = file.as_slice();
        // NKSHeader::read_le(file.as_slice())?;
        println!("{:?}", NKSHeader::read_le(file.as_slice())?);
        Ok(())
    }

    #[test]
    fn test_header_v42_read() -> Result<(), NIFileError> {
        let file = include_bytes!("../../tests/patchdata/NKS/BPatchHeaderV42/000");
        println!("{:?}", NKSHeader::read_le(file.as_slice())?);
        Ok(())
    }
}
