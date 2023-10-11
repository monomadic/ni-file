// props:
// - masterVolume float
// - masterTune float
// - masterTempo uint32_t
// - name wstring

use std::io::Cursor;

use crate::{
    kontakt::{structured_object::StructuredObject, Chunk, KontaktError},
    read_bytes::ReadBytesExt,
    Error,
};

#[derive(Debug)]
pub struct Bank(StructuredObject);

#[derive(Debug)]
pub struct BankPublicParams {
    master_volume: f32,
    master_tune: f32,
    master_tempo: i32,
    name: String,
}

impl Bank {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let so = StructuredObject::read(&mut reader)?;
        Ok(Self(so))
    }

    pub fn params(&self) -> Result<BankPublicParams, Error> {
        let mut reader = Cursor::new(&self.0.public_data);
        Ok(BankPublicParams {
            master_volume: reader.read_f32_le()?,
            master_tune: reader.read_f32_le()?,
            master_tempo: reader.read_i32_le()?,
            name: reader.read_widestring_utf16()?,
        })
    }
}

impl std::convert::TryFrom<&Chunk> for Bank {
    type Error = Error;

    fn try_from(chunk: &Chunk) -> Result<Self, Self::Error> {
        if chunk.id != 0x03 {
            return Err(KontaktError::IncorrectID {
                expected: 0x03,
                got: chunk.id,
            }
            .into());
        }
        let reader = Cursor::new(&chunk.data);
        Self::read(reader)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use crate::Error;

    use super::*;

    #[test]
    fn test_bank() -> Result<(), Error> {
        let chunk: Chunk = Chunk::read(File::open("tests/data/Objects/Kontakt/Bank/Bank-000")?)?;
        let bank = Bank::try_from(&chunk)?;
        dbg!(bank.params()?);
        for chunk in bank.0.children {
            println!("{:?} {:x}", chunk.into_type()?, chunk.id);
        }
        Ok(())
    }
}
