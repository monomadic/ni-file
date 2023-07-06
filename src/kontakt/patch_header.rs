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
/// | Offset | Length | Type     | Meaning                     | Default    | Notes                                    |
/// |--------|--------|----------|-----------------------------|------------|------------------------------------------|
/// | 0x00   | 0x04   | uint32_t | magic                       | 0x1290A87F | 0xa4d6e55a || 0xab85ef01 || 0xb36ee55e || 0x10874353 ||  0x74b5a69b || 0x7fa89012                                        |
/// | 0x04   | 0x04   | uint32_t | zLibLength                  |            | Internal preset compressed size          |
/// | 0x08   | 0x02   | uint16_t | headerVersion               | 0x1001     | Found 272                                |
/// | 0x0A   | 0x04   | uint32_t | patchVersion                | 0x1A6337EA |                                          |
/// | 0x0E   | 0x02   | uint16_t | patchtype                   | 0x1 (nki)  | 0=nkm, 1=nki, 2=nkb, 3=nkp, 4=nkg, nkz=5 |
/// | 0x10   | 0x04   | AppVersi | appVersion                  | 0x50500ff  |                                          |
/// | 0x14   | 0x04   | uint32_t | appSignature                | 0x4b34504c | "Kon4"                                   |
/// | 0x18   | 0x04   | time_t   | createdAt                   |            |                                          |
/// | 0x1C   | 0x04   |          |                             |            |                                          |
/// | 0x20   | 0x02   | uint16_t | numZones                    |            |                                          |
/// | 0x22   | 0x02   | uint16_t | numGroups                   |            |                                          |
/// | 0x24   | 0x02   | uint16_t | numInstruments              |            |                                          |
/// | 0x26   | 0x10   |          |                             |            |                                          |
/// | 0x36   | 0x10   | uint32_t | icon                        |            | 0x1C is "New"                            |
/// |        |        |          |                             |            |                                          |
/// | 0xA2   | 0x10   |          | checksum                    |            |                                          |
/// | 0xB2   | 0x04   | uint32_t | appSVNRev                   |            |                                          |
/// | 0xB6   | 0x04   | uint32_t |                             |            |                                          |
/// | 0xBA   | 0x04   | uint32_t | decompressedLength          |            |                                          |
/// |        | 0x20   |          |                             |            |                                          |
///
#[derive(Debug)]
pub struct BPatchHeaderV42 {
    pub zlib_length: usize,
    pub decompressed_length: usize,
    pub patch_version: u16,
    pub app_version: AppVersionV42,
}

#[derive(Debug)]
pub struct AppVersionV42 {
    major: u8,
    minor_1: u8,
    minor_2: u8,
    minor_3: u8,
}

impl BPatchHeaderV42 {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, NIFileError> {
        let magic = reader.read_u32_le()?;
        assert_eq!(magic, u32::swap_bytes(0x1290A87F));

        let zlib_length = reader.read_u32_le()? as usize;
        let patch_version = reader.read_u16_le()?;
        assert_eq!(patch_version, u16::swap_bytes(0x1001));

        let magic2 = reader.read_u32_le()?;
        assert_eq!(magic2, u32::swap_bytes(0x1A6337EA));
        // println!("magic2 0x{:x}", magic2);

        let preset_type = reader.read_u16_le()?;
        println!("preset_type {}", preset_type);

        let app_version = AppVersionV42 {
            minor_3: reader.read_u8()?,
            minor_2: reader.read_u8()?,
            minor_1: reader.read_u8()?,
            major: reader.read_u8()?,
        };

        println!(
            "app_version {}.{}.{}.{}",
            app_version.major, app_version.minor_2, app_version.minor_2, app_version.minor_3
        );

        let app_signature = reader.read_u32_le()?;
        println!("app_signature {}", app_signature);

        let datetime = OffsetDateTime::from_unix_timestamp(reader.read_u32_le()? as i64).unwrap();
        let created_at: time::Date = datetime.date();
        println!("created_at {}", created_at);

        let _unknown = reader.read_u32_le()?;

        let number_of_zones = reader.read_u16_le()?;
        let number_of_groups = reader.read_u16_le()?;
        let number_of_instruments = reader.read_u16_le()?;
        println!(
            "zones: {} groups: {} instruments: {}",
            number_of_zones, number_of_groups, number_of_instruments
        );

        let _unknown = reader.read_bytes(16)?;

        let icon = reader.read_u32_le()?;
        println!("icon {}", icon);

        println!("author {}", reader.read_string_utf8()?);

        let _unknown = reader.read_bytes(101)?;

        let _checksum = reader.read_bytes(16)?;

        let _unknown = reader.read_u32_le()?;
        let _unknown = reader.read_u32_le()?;

        let decompressed_length = reader.read_u32_le()? as usize;
        println!("decompressed_length {}", decompressed_length);

        // seems all zero bytes
        let _unknown = reader.read_bytes(32)?;

        Ok(Self {
            zlib_length,
            decompressed_length,
            patch_version,
            app_version,
        })
    }
}
