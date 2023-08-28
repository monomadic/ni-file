use std::io::Cursor;

use crate::{read_bytes::ReadBytesExt, Error};

use super::{
    filename_list::FNTableImpl, objects::bprogram::Program, structured_object::StructuredObject,
};

#[derive(Debug)]
pub struct ChunkData {
    pub id: u16,
    pub data: Vec<u8>,
}

impl ChunkData {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let id = reader.read_u16_le()?;
        let length = reader.read_u32_le()? as usize;
        let data = reader.read_bytes(length)?;
        Ok(Self { id, data })
    }
}

impl std::convert::TryFrom<ChunkData> for StructuredObject {
    type Error = Error;

    fn try_from(chunk: ChunkData) -> Result<Self, Self::Error> {
        let cursor = Cursor::new(chunk.data);
        Ok(StructuredObject::read(cursor)?)
    }
}

impl TryFrom<ChunkData> for Chunk {
    type Error = Error;

    fn try_from(chunk: ChunkData) -> Result<Chunk, Error> {
        let reader = Cursor::new(chunk.data);

        Ok(match chunk.id {
            0x28 => Chunk::Program(Program::read(reader)?),
            0x4b => Chunk::FNTableImpl(FNTableImpl::read(reader)?),
            _ => Chunk::Unsupported(chunk.id),
        })
    }
}

#[derive(Debug)]
pub enum Chunk {
    Program(Program),
    StructuredObject(StructuredObject),
    FNTableImpl(FNTableImpl),
    Unsupported(u16),
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn test_structured_object() -> Result<(), Error> {
        let file = File::open("tests/patchdata/KontaktV42/StructuredObject/0x28")?;
        let data = ChunkData::read(file)?;
        let chunk: Chunk = data.try_into()?;

        dbg!(chunk);

        Ok(())
    }

    #[test]
    fn test_fntableimpl() -> Result<(), Error> {
        let file = File::open("tests/patchdata/KontaktV42/FNTableImpl/FNTableImpl-001")?;
        let data = ChunkData::read(file)?;
        let chunk: Chunk = data.try_into()?;

        dbg!(chunk);

        Ok(())
    }
}
