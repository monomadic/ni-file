use std::io::Cursor;

use crate::{read_bytes::ReadBytesExt, Error};

use super::{
    objects::{program::Program, Bank, FNTableImpl},
    structured_object::StructuredObject,
};

#[derive(Debug)]
pub struct Chunk {
    pub id: u16,
    pub data: Vec<u8>,
}

impl Chunk {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let id = reader.read_u16_le()?;
        let length = reader.read_u32_le()? as usize;
        let data = reader.read_bytes(length)?;
        Ok(Self { id, data })
    }

    pub fn into_type(&self) -> Result<KontaktObject, Error> {
        Ok(KontaktObject::try_from(self)?)
    }
}

impl std::convert::TryFrom<&Chunk> for StructuredObject {
    type Error = Error;

    fn try_from(chunk: &Chunk) -> Result<Self, Self::Error> {
        let cursor = Cursor::new(&chunk.data);
        Ok(StructuredObject::read(cursor)?)
    }
}

#[derive(Debug)]
pub enum KontaktObject {
    Bank(Bank),
    BParScript,
    Program(Program),
    StructuredObject(StructuredObject),
    SaveSettings,
    FNTableImpl(FNTableImpl),
    Unsupported(u16),
    BOutputConfiguration,
}

impl TryFrom<&Chunk> for KontaktObject {
    type Error = Error;

    fn try_from(chunk: &Chunk) -> Result<KontaktObject, Error> {
        let reader = Cursor::new(&chunk.data);

        Ok(match chunk.id {
            0x03 => KontaktObject::Bank(Bank::read(reader)?),
            0x06 => KontaktObject::BParScript,
            0x28 => KontaktObject::Program(Program::read(reader)?),
            0x3e => KontaktObject::BOutputConfiguration,
            0x47 => KontaktObject::SaveSettings,
            0x4b => KontaktObject::FNTableImpl(FNTableImpl::read(reader)?),
            _ => KontaktObject::Unsupported(chunk.id),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn test_structured_object() -> Result<(), Error> {
        let file = File::open("tests/data/Objects/KontaktV42/StructuredObject/0x28")?;
        let data = Chunk::read(file)?;
        let chunk: KontaktObject = (&data).try_into()?;

        dbg!(chunk);

        Ok(())
    }

    #[test]
    fn test_fntableimpl() -> Result<(), Error> {
        let file = File::open("tests/data/Objects/KontaktV42/FNTableImpl/FNTableImpl-001")?;
        let data = Chunk::read(file)?;
        let chunk: KontaktObject = (&data).try_into()?;

        dbg!(chunk);

        Ok(())
    }
}
