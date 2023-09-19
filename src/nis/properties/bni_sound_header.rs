use crate::{nks::header::BPatchHeaderV42, read_bytes::ReadBytesExt, Error};

/// Kontakt header
#[derive(Debug)]
pub struct BNISoundHeader(pub BPatchHeaderV42);

impl BNISoundHeader {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let magic = reader.read_u32_le()?;
        assert_eq!(magic, 0x7fa89012);

        let _zlib_length = reader.read_u32_le()?;

        let header_version = reader.read_u16_le()?;
        assert_eq!(header_version, 0x0110);

        let header = BPatchHeaderV42::read_le(&mut reader)?;

        Ok(Self(header))
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn test_bni_sound_header_read() -> Result<(), Error> {
        let file = File::open("test-data/NIS/Objects/BNISoundHeader/000")?;
        println!("{:?}", BNISoundHeader::read(file)?);
        Ok(())
    }
}