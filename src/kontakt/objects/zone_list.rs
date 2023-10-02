use crate::{
    kontakt::{chunk::Chunk, error::KontaktError},
    read_bytes::ReadBytesExt,
    Error,
};

use super::zone_data::ZoneData;

#[derive(Debug)]
pub struct ZoneList {
    pub zones: Vec<ZoneData>,
}

impl ZoneList {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let zone_count = reader.read_u32_le()?;

        let mut zones = Vec::new();
        for _ in 0..zone_count {
            let _unknown = reader.read_u32_le()?;
            zones.push(ZoneData::read(&mut reader)?);
        }

        Ok(Self { zones })
    }
}

impl std::convert::TryFrom<&Chunk> for ZoneList {
    type Error = Error;

    fn try_from(chunk: &Chunk) -> Result<Self, Self::Error> {
        if chunk.id != 0x34 {
            return Err(KontaktError::IncorrectID {
                expected: 0x34,
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
    fn test_zone_list_empty() -> Result<(), Error> {
        let file = File::open("tests/patchdata/KontaktV42/ZoneList/ZoneList-000")?;
        let zonelist = ZoneList::read(file)?;
        assert_eq!(zonelist.zones.len(), 0);
        Ok(())
    }

    #[test]
    fn test_zone_list_001() -> Result<(), Error> {
        let file = File::open("tests/patchdata/KontaktV42/ZoneList/ZoneList-001")?;
        let zonelist = ZoneList::read(file)?;
        assert_eq!(zonelist.zones.len(), 61);
        Ok(())
    }

    #[test]
    fn test_zone_list_002() -> Result<(), Error> {
        let file = File::open("tests/patchdata/KontaktV42/ZoneList/ZoneList-002")?;
        let zonelist = ZoneList::read(file)?;
        assert_eq!(zonelist.zones.len(), 31);
        Ok(())
    }

    #[test]
    fn test_zone_list_003() -> Result<(), Error> {
        let file = File::open("tests/patchdata/KontaktV42/ZoneList/ZoneList-003")?;
        let zonelist = ZoneList::read(file)?;
        assert_eq!(zonelist.zones.len(), 32);
        Ok(())
    }
}
