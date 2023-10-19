use crate::{
    kontakt::{Chunk, KontaktError},
    read_bytes::ReadBytesExt,
    Error,
};

const CHUNK_ID: u16 = 0x47;

/// Type:           Chunk
/// SerType:        0x47
/// Versions:       0x10
/// Kontakt 7:      SaveSettings
/// KontaktIO:      ?
#[derive(Debug)]
pub struct SaveSettings;

// SER::WriteBFNTrns(param_1,(BFileName *)(this + 8),param_2);
//      i32 numFiles BFileName
// SER::WriteBFNOrig(param_1,(BFileName *)(this + 0x130),param_2);
//      i32 numFiles BFileName
// i32
// bool
// bool
// bool

impl SaveSettings {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let is_structured = reader.read_bool()?; // is structured
        assert!(!is_structured);

        dbg!(reader.read_u16_le()?);
        dbg!(reader.read_i32_le()?);
        dbg!(reader.read_i32_le()?);

        dbg!(reader.read_i32_le()?);
        dbg!(reader.read_bool()?);
        dbg!(reader.read_bool()?);
        dbg!(reader.read_bool()?);
        Ok(Self)
    }
}

impl std::convert::TryFrom<&Chunk> for SaveSettings {
    type Error = Error;

    fn try_from(chunk: &Chunk) -> Result<Self, Self::Error> {
        if chunk.id != CHUNK_ID {
            return Err(KontaktError::IncorrectID {
                expected: CHUNK_ID,
                got: chunk.id,
            }
            .into());
        }
        let mut reader = std::io::Cursor::new(&chunk.data);
        Self::read(&mut reader)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{kontakt::Chunk, Error};
    use std::fs::File;

    #[test]
    fn test_bank() -> Result<(), Error> {
        let chunk = Chunk::read(File::open(
            "tests/data/Objects/Kontakt/0x47-SaveSettings/SaveSettings-000.kon",
        )?)?;
        let _s = SaveSettings::try_from(&chunk)?;

        Ok(())
    }
}
