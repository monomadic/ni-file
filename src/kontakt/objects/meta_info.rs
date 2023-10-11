use crate::{nks::error::NKSError, read_bytes::ReadBytesExt};

/// The header of the metadata (footer) of a Kontakt42 preset.
///
/// | Offset | Length | Type     | Meaning                     | Default    | Notes                                    |
/// |--------|--------|----------|-----------------------------|------------|------------------------------------------|
/// | 0x00   | 0x04   | uint32_t | metaMagic                   | 0xAEE10EB0 | BE: 0xb00ee1ae                               |
#[derive(Debug)]
pub struct BPatchMetaInfoHeader {
    /// An XML SoundInfoItem document.
    soundinfo: String,
}

impl BPatchMetaInfoHeader {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, NKSError> {
        let magic: u32 = reader.read_le()?;

        assert_eq!(
            magic, 0xB00EE1AE,
            "Invalid BPatchMetaInfoHeader magic number: expected 0xaee10eb0 got 0x{magic:X}"
        );

        // unknown
        assert_eq!(reader.read_u8()?, 1);
        assert_eq!(reader.read_u8()?, 1);

        // chunk id
        assert_eq!(reader.read_u16_le()?, 12);

        let soundinfo_length = reader.read_u32_le()? as usize;
        let soundinfo = reader.read_bytes(soundinfo_length)?;
        let soundinfo = String::from_utf8(soundinfo).unwrap();

        Ok(Self { soundinfo })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_kontakt_1() -> Result<(), NKSError> {
        BPatchMetaInfoHeader::read(File::open(
            "tests/data/Objects/BPatchMetaInfoHeader/BPatchMetaInfoHeader-000",
        )?)?;
        Ok(())
    }
}
