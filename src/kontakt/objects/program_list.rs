use crate::{
    kontakt::{Chunk, KontaktError},
    read_bytes::ReadBytesExt,
    Error,
};

use super::Program;

/// Type:           Chunk
/// SerType:        0x36
/// Kontakt 7:      ?
/// KontaktIO:      ProgramList
#[derive(Debug)]
pub struct ProgramList {
    pub programs: Vec<Program>,
}

impl ProgramList {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let num_programs = reader.read_i16_le()?;
        let mut programs = Vec::new();

        for _ in 0..num_programs {
            let chunk = Chunk::read(&mut reader)?;
            programs.push((&chunk).try_into()?);
        }

        Ok(Self { programs })
    }
}

impl std::convert::TryFrom<&Chunk> for ProgramList {
    type Error = Error;

    fn try_from(chunk: &Chunk) -> Result<Self, Self::Error> {
        if chunk.id != 0x36 {
            return Err(KontaktError::IncorrectID {
                expected: 0x36,
                got: chunk.id,
            }
            .into());
        }
        let mut reader = std::io::Cursor::new(&chunk.data);
        Self::read(&mut reader)
    }
}
