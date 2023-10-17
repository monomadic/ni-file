use std::io::Cursor;

use crate::{
    kontakt::{Chunk, KontaktError, StructuredObject},
    read_bytes::ReadBytesExt,
    Error,
};

/// ProgramContainer
///
/// - SerType:        0x29
/// - Known Versions: 0x50, 0x51
/// - Kontakt 7:      BProgramContainer
/// - KontaktIO:      K4PL\_ProgramContainer
#[derive(Debug)]
pub struct ProgramContainer(pub StructuredObject);

#[derive(Debug)]
pub struct ProgramContainerParams {
    name: String,
    volume: f32,
    pan: f32,
}

impl ProgramContainer {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let so = StructuredObject::read(&mut reader)?;

        Ok(Self(so))
    }

    pub fn params(&self) -> Result<ProgramContainerParams, Error> {
        let mut reader = Cursor::new(&self.0.public_data);

        Ok(ProgramContainerParams {
            name: reader.read_widestring_utf16()?,
            volume: reader.read_f32_le()?,
            pan: reader.read_f32_le()?,
        })
    }
}

impl std::convert::TryFrom<&Chunk> for ProgramContainer {
    type Error = Error;

    fn try_from(chunk: &Chunk) -> Result<Self, Self::Error> {
        if chunk.id != 0x29 {
            return Err(KontaktError::IncorrectID {
                expected: 0x29,
                got: chunk.id,
            }
            .into());
        }
        let reader = Cursor::new(&chunk.data);
        Self::read(reader)
    }
}
