use crate::{
    kontakt::{Chunk, KontaktError, StructuredObject},
    read_bytes::ReadBytesExt,
    Error,
};

use super::Program;

const CHUNK_ID: u16 = 0x36;

/// ProgramList
///
/// An array of `Program` objects.
///
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
            let _ = reader.read_i16_le()?;
            let so = Program(StructuredObject::read(&mut reader)?);
            programs.push(so);
        }

        Ok(Self { programs })
    }
}

impl std::convert::TryFrom<&Chunk> for ProgramList {
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
    use crate::Error;
    use std::fs::File;

    #[test]
    fn test_bank() -> Result<(), Error> {
        let chunk = Chunk::read(File::open(
            "tests/data/Objects/Kontakt/0x36-ProgramList/ProgramList-000.kon",
        )?)?;
        let _pl = ProgramList::try_from(&chunk)?;

        Ok(())
    }
}
