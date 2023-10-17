use std::io::Cursor;

use crate::{
    kontakt::{structured_object::StructuredObject, Chunk, KontaktError},
    read_bytes::ReadBytesExt,
    Error,
};

pub const KONTAKT_BANK_ID: u16 = 0x03;

/// Type:           Chunk
/// SerType:        0x03
/// Known Versions: 0x60, 0x71, 0x72, 0x73
/// Kontakt 7:      BBank
/// KontaktIO:      K4PL\_Bank
///
/// Children:
///   SaveSettings 0x47
///   BParScript 0x6
///   BParScript 0x6
///   BParScript 0x6
///   BParScript 0x6
///   BParScript 0x6
///   BOutputConfiguration 0x3e
///   SlotList 0x37
///   Unsupported(72) 0x48
///   Unsupported(73) 0x49
///
///   ProgramContainer? 0x29
///   BMidiObject?
///   BMultiChannelProcessBuffer?
#[derive(Debug)]
pub struct Bank(StructuredObject);

#[derive(Debug)]
pub struct BankPublicParams {
    pub master_volume: f32,
    pub master_tune: f32,
    pub master_tempo: i32,
    pub name: String,
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

    pub fn slot_list(&self) -> Result<super::SlotList, Error> {
        (&self.0.children[7]).try_into()
    }
}

impl std::convert::TryFrom<&Chunk> for Bank {
    type Error = Error;

    fn try_from(chunk: &Chunk) -> Result<Self, Self::Error> {
        // match KontaktObject::try_from(chunk)? {
        //     KontaktObject::BBank(bank) => Ok(bank),
        //     _ => Err(KontaktError::IncorrectID {
        //         expected: 0x03,
        //         got: chunk.id,
        //     }.into())
        // }
        if chunk.id != KONTAKT_BANK_ID {
            return Err(KontaktError::IncorrectID {
                expected: KONTAKT_BANK_ID,
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
    use super::*;
    use crate::Error;
    use std::fs::File;

    #[test]
    fn test_bank() -> Result<(), Error> {
        let chunk = Chunk::read(File::open(
            "tests/data/Objects/Kontakt/Bank/BankV73-000.kon",
        )?)?;
        let bank = Bank::try_from(&chunk)?;
        dbg!(bank.0.version);
        dbg!(bank.params()?);
        for chunk in bank.0.children {
            println!("{:?} 0x{:x}", chunk.into_object()?, chunk.id);

            // let filename = format!("{:?}-{:x}.chunk", chunk.into_type()?, chunk.id);
            // std::fs::write(filename, chunk.data)?;
        }
        Ok(())
    }
}
