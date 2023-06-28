use crate::{read_bytes::ReadBytesExt, NIFileError};

/// The header of a Kontakt42 preset.
/// 178 bytes?
///
/// | Offset | Length | Type     | Meaning                     | Default    | Other                                    |
/// |--------|--------|----------|-----------------------------|------------|------------------------------------------|
/// | 0      | 4      | uint32_t | magic                       | 0x1290A87F |                                          |
/// | 4      | 4      | uint32_t | zLibLength                  | 0          |                                          |
/// | 8      | 2      | uint16_t | fileVersion (must be 0x110) | 0x110      |                                          |
/// | 10     | 4      | uint32_t | version                     | 0xea37631a |                                          |
/// | 14     | 2      | uint16_t | type                        | 1 (nki)    | 0=nkm, 1=nki, 2=nkb, 3=nkp, 4=nkg, nkz=5 |
/// | 16     | 4      | uint32_t | appVersion                  | 0x50500ff  |                                          |
/// | 0x42   | 4      | uint32_t | appSignature                | 0x4b34504c |                                          |
/// | 24     | 4      | time_t   | creationTime                |            |                                          |
/// | ..     |        |          |                             |            |                                          |
/// | 178    | 4      | uint32_t | appSVNRev                   |            |                                          |
pub struct BPatchHeaderV42;

impl BPatchHeaderV42 {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, NIFileError> {
        let magic = reader.read_u32_le()?;
        assert_eq!(magic, u32::swap_bytes(0x1290A87F));
        println!("magic 0x{:x}", magic);

        let zlib_length = reader.read_u32_le()?;
        println!("zlib_length {}", zlib_length);

        let file_version = reader.read_u16_le()?;
        assert_eq!(file_version, u16::swap_bytes(0x1001));
        println!("file_version 0x{:x}", file_version);

        let magic2 = reader.read_u32_le()?;
        assert_eq!(magic2, u32::swap_bytes(0x1A6337EA));
        println!("magic2 0x{:x}", magic2);

        let preset_type = reader.read_u16_le()?;
        println!("preset_type {}", preset_type);

        let app_version = reader.read_u32_le()?;
        println!("app_version {}", app_version);

        let app_signature = reader.read_u32_le()?;
        println!("app_signature {}", app_signature);

        Ok(Self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kontakt42_preset_read() -> Result<(), NIFileError> {
        //crate::utils::setup_logger();

        for path in crate::utils::get_files("../tests/data/kontakt42/**/*.nki")? {
            println!("reading {:?}", path);

            let file = std::fs::File::open(&path)?;
            BPatchHeaderV42::read(file)?;
        }

        Ok(())
    }
}
