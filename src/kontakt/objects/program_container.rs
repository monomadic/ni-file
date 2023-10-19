use std::io::Cursor;

use crate::{
    kontakt::{Chunk, KontaktError, StructuredObject},
    read_bytes::ReadBytesExt,
    Error,
};

use super::{ProgramList, VoiceGroup};

const CHUNK_ID: u16 = 0x29;

/// # ProgramContainer
///
/// - Type:           Chunk<StructuredObject>
/// - SerType:        0x29
/// - Versions:       0x50, 0x51
/// - Kontakt 7:      BProgramContainer
/// - KontaktIO:      K4PL\_ProgramContainer
///
#[derive(Debug)]
pub struct ProgramContainer(pub StructuredObject);

#[derive(Debug)]
pub struct ProgramContainerParams {
    name: String,
    volume: f32,
    pan: f32,
}

impl ProgramContainer {
    pub fn params(&self) -> Result<ProgramContainerParams, Error> {
        let mut reader = Cursor::new(&self.0.public_data);

        Ok(ProgramContainerParams {
            name: reader.read_widestring_utf16()?,
            volume: reader.read_f32_le()?,
            pan: reader.read_f32_le()?,
        })
    }

    pub fn voice_group(&self) -> Result<VoiceGroup, Error> {
        (&self.0.children[0]).try_into()
    }

    // pub fn save_settings(&self) -> Result<SaveSettings, Error> {
    //     (&self.0.children[1]).try_into()
    // }

    pub fn program_list(&self) -> Result<ProgramList, Error> {
        (&self.0.children[2]).try_into()
    }

    // pub fn programs(&self) -> Result<Vec<VoiceGroup>, Error> {
    //     self.0.children.iter().map(VoiceGroup::try_from).collect()
    // }
}

impl std::convert::TryFrom<&Chunk> for ProgramContainer {
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
    use super::*;
    use crate::Error;
    use std::fs::File;

    #[test]
    fn test_bank() -> Result<(), Error> {
        let chunk = Chunk::read(File::open(
            "tests/data/Objects/Kontakt/0x29-ProgramContainer/ProgramContainerV51-000.kon",
        )?)?;
        let pc = ProgramContainer::try_from(&chunk)?;

        assert_eq!(pc.0.version, 0x51);
        assert_eq!(pc.0.children.len(), 3);
        assert_eq!(pc.0.children[0].id, 0x2B); // VoiceGroup
        assert_eq!(pc.0.children[1].id, 0x47); // SaveSettings
        assert_eq!(pc.0.children[2].id, 0x36); // ProgramList

        pc.params()?;
        pc.voice_group()?;
        pc.program_list()?;
        // println!("{:X}", pc.0.children[2].id);
        Ok(())
    }
}
