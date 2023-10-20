use std::io::Cursor;

use crate::{
    kontakt::{Chunk, KontaktError, StructuredObject},
    read_bytes::ReadBytesExt,
    Error,
};

const CHUNK_ID: u16 = 0x10;

/// # FXDelay
///
/// Delay effect.
///
/// - Type:           Chunk<StructuredObject>
/// - SerType:        0x10
/// - Versions:       0x50, 0x51
/// - Kontakt 7:      BParFXDelay
/// - KontaktIO:      ?
///
#[derive(Debug)]
pub struct FXDelay(pub StructuredObject);

#[derive(Debug)]
pub struct FXDelayParams {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    e: f32,
    f: f32,
    g: f32,
    h: bool,
}

impl FXDelay {
    pub fn params(&self) -> Result<FXDelayParams, Error> {
        let mut reader = Cursor::new(&self.0.public_data);

        Ok(FXDelayParams {
            a: reader.read_f32_le()?,
            b: reader.read_f32_le()?,
            c: reader.read_f32_le()?,
            d: reader.read_f32_le()?,
            e: reader.read_f32_le()?,
            f: reader.read_f32_le()?,
            g: reader.read_f32_le()?,
            h: reader.read_bool()?,
        })
    }
}

impl std::convert::TryFrom<&Chunk> for FXDelay {
    type Error = Error;

    fn try_from(chunk: &Chunk) -> Result<Self, Self::Error> {
        if chunk.id != CHUNK_ID {
            return Err(KontaktError::IncorrectID {
                expected: CHUNK_ID,
                got: chunk.id,
            }
            .into());
        }
        Ok(Self(chunk.try_into()?))
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::Error;
    // use std::fs::File;
    //
    // #[test]
    // fn test_bank() -> Result<(), Error> {
    //     let chunk = Chunk::read(File::open(
    //         "tests/data/Objects/Kontakt/0x29-ProgramContainer/ProgramContainerV51-000.kon",
    //     )?)?;
    //     let pc = InsertBus::try_from(&chunk)?;
    //
    //     assert_eq!(pc.0.version, 0x51);
    //     assert_eq!(pc.0.children.len(), 3);
    //     assert_eq!(pc.0.children[0].id, 0x2B); // VoiceGroup
    //     assert_eq!(pc.0.children[1].id, 0x47); // SaveSettings
    //     assert_eq!(pc.0.children[2].id, 0x36); // ProgramList
    //
    //     pc.params()?;
    //     pc.voice_group()?;
    //     pc.program_list()?;
    //     // println!("{:X}", pc.0.children[2].id);
    //     Ok(())
    // }
}
