use std::io::Cursor;

use crate::{read_bytes::ReadBytesExt, Error};

use super::{
    objects::{filename_list::FNTableImpl, program::Program},
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
}

impl std::convert::TryFrom<Chunk> for StructuredObject {
    type Error = Error;

    fn try_from(chunk: Chunk) -> Result<Self, Self::Error> {
        let cursor = Cursor::new(chunk.data);
        Ok(StructuredObject::read(cursor)?)
    }
}

#[derive(Debug)]
pub enum ChunkType {
    Program(Program),
    StructuredObject(StructuredObject),
    FNTableImpl(FNTableImpl),
    Unsupported(u16),
}

impl TryFrom<Chunk> for ChunkType {
    type Error = Error;

    fn try_from(chunk: Chunk) -> Result<ChunkType, Error> {
        let reader = Cursor::new(chunk.data);

        Ok(match chunk.id {
            0x28 => ChunkType::Program(Program::read(reader)?),
            0x4b => ChunkType::FNTableImpl(FNTableImpl::read(reader)?),
            _ => ChunkType::Unsupported(chunk.id),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn test_structured_object() -> Result<(), Error> {
        let file = File::open("tests/patchdata/KontaktV42/StructuredObject/0x28")?;
        let data = Chunk::read(file)?;
        let chunk: ChunkType = data.try_into()?;

        dbg!(chunk);

        Ok(())
    }

    #[test]
    fn test_fntableimpl() -> Result<(), Error> {
        let file = File::open("tests/patchdata/KontaktV42/FNTableImpl/FNTableImpl-001")?;
        let data = Chunk::read(file)?;
        let chunk: ChunkType = data.try_into()?;

        dbg!(chunk);

        Ok(())
    }
}
