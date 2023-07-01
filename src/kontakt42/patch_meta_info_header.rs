use crate::{read_bytes::ReadBytesExt, NIFileError};

/// The header of the metadata (footer) of a Kontakt42 preset.
///
/// | Offset | Length | Type     | Meaning                     | Default    | Notes                                    |
/// |--------|--------|----------|-----------------------------|------------|------------------------------------------|
/// | 0x00   | 0x04   | uint32_t | magic                       | 0xAEE10EB0 |                                          |
pub struct BPatchMetaInfoHeader {
    /// An XML SoundInfoItem document.
    soundinfo: String
}

impl BPatchMetaInfoHeader {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, NIFileError> {
        // magic
        assert_eq!(reader.read_u32_le()?, u32::swap_bytes(0xAEE10EB0));

        // unknown
        assert_eq!(reader.read_u8()?, 1);
        assert_eq!(reader.read_u8()?, 1);

        // chunk id
        assert_eq!(reader.read_u16_le()?, 12);

        let soundinfo_length = reader.read_u32_le()? as usize;
        let soundinfo = reader.read_bytes(soundinfo_length)?;
        let soundinfo = String::from_utf8(soundinfo).unwrap();

        Ok( Self { soundinfo })
    }
}
