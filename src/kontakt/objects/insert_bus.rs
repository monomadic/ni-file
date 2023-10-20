use std::io::Cursor;

use crate::{
    kontakt::{Chunk, KontaktError, StructuredObject},
    read_bytes::ReadBytesExt,
    Error,
};

const CHUNK_ID: u16 = 0x45;

/// # InsertBus
///
/// Also known as 'Instrument Buses'. Unlike send buses, these completely
/// replace an audio channel with the processed signal.
///
/// - Type:           Chunk<StructuredObject>
/// - SerType:        0x45
/// - Versions:       0x11
/// - Kontakt 7:      BInsertBus
/// - KontaktIO:      ?
///
#[derive(Debug)]
pub struct InsertBus(pub StructuredObject);

#[derive(Debug)]
pub struct InsertBusParams {
    name: String,
    pan: f32,
    volume: f32,
    output: i32,
}

impl InsertBus {
    pub fn params(&self) -> Result<InsertBusParams, Error> {
        let mut reader = Cursor::new(&self.0.public_data);

        Ok(InsertBusParams {
            name: reader.read_widestring_utf16()?,
            pan: reader.read_f32_le()?,
            volume: reader.read_f32_le()?,
            output: reader.read_i32_le()?,
        })
    }
}

impl std::convert::TryFrom<&Chunk> for InsertBus {
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
