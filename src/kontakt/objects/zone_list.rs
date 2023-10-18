use crate::{
    kontakt::{chunk::Chunk, error::KontaktError},
    read_bytes::ReadBytesExt,
    Error,
};

use super::Zone;

const CHUNK_ID: u16 = 0x34;

/// Type:           Chunk
/// SerType:        0x34
/// Kontakt 7:      BZoneArraySer, BProgram::readZones()
/// KontaktIO:      ZoneList<K4PL_Zone<K4PO::K4PL_ZoneDataV95>>
#[derive(Debug)]
pub struct ZoneList {
    pub zones: Vec<Zone>,
}

impl ZoneList {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let num_zones = reader.read_u32_le()?;
        let mut zones = Vec::new();

        for _ in 0..num_zones {
            let _ = reader.read_u32_le()?;
            zones.push(Zone::read(&mut reader)?);
        }

        Ok(Self { zones })
    }
}

impl std::convert::TryFrom<&Chunk> for ZoneList {
    type Error = Error;

    fn try_from(chunk: &Chunk) -> Result<Self, Self::Error> {
        if chunk.id != CHUNK_ID {
            return Err(KontaktError::IncorrectID {
                expected: CHUNK_ID,
                got: chunk.id,
            }
            .into());
        }
        let reader = std::io::Cursor::new(&chunk.data);
        Self::read(reader)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;
    use crate::Error;

    #[test]
    fn test_zone_list_001() -> Result<(), Error> {
        let file = File::open("tests/data/Objects/Kontakt/0x34-ZoneList/old/ZoneList-001.kon")?;
        let zonelist = ZoneList::read(file)?;
        assert_eq!(zonelist.zones.len(), 61);
        Ok(())
    }

    #[test]
    fn test_zone_list_002() -> Result<(), Error> {
        let file = File::open("tests/data/Objects/Kontakt/0x34-ZoneList/old/ZoneList-002.kon")?;
        let zonelist = ZoneList::read(file)?;
        assert_eq!(zonelist.zones.len(), 31);
        Ok(())
    }

    #[test]
    fn test_zone_list_003() -> Result<(), Error> {
        let file = File::open("tests/data/Objects/Kontakt/0x34-ZoneList/old/ZoneList-003.kon")?;
        let zonelist = ZoneList::read(file)?;
        assert_eq!(zonelist.zones.len(), 32);
        Ok(())
    }
}
