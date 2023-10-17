// Groups allow you to apply settings like effects, volume, panning, etc. to multiple samples at once rather than having to adjust each one individually.

use crate::{
    kontakt::{Chunk, KontaktError, StructuredObject},
    read_bytes::ReadBytesExt,
    Error,
};

use super::Group;

pub const KONTAKT_GROUPLIST_ID: u16 = 0x33;

/// Type:           Chunk
/// SerType:        0x33
/// Kontakt 7:      BProgram::readGroups()
/// KontaktIO:      GroupList
#[derive(Debug)]
pub struct GroupList {
    groups: Vec<Group>,
}

impl GroupList {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let num_groups = reader.read_u32_le()?;
        let mut groups = Vec::new();

        for _ in 0..num_groups {
            let group = Group(StructuredObject::read(&mut reader)?);
            group.params()?;
            groups.push(group);
        }

        Ok(Self { groups })
    }
}

impl std::convert::TryFrom<&Chunk> for GroupList {
    type Error = Error;

    fn try_from(chunk: &Chunk) -> Result<Self, Self::Error> {
        if chunk.id != KONTAKT_GROUPLIST_ID {
            return Err(KontaktError::IncorrectID {
                expected: KONTAKT_GROUPLIST_ID,
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
    use std::{fs::File, io::Read};

    use super::*;

    #[test]
    fn test_group_list_000() -> Result<(), Error> {
        let mut file = File::open("tests/data/Objects/Kontakt/GroupList/GroupList-000")?;

        GroupList::try_from(&Chunk::read(&mut file)?)?;

        // Ensure the read completed
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        assert_eq!(buf.len(), 0, "Excess data found: {} bytes", buf.len());

        Ok(())
    }

    #[test]
    fn test_group_list_001() -> Result<(), Error> {
        let mut file = File::open("tests/data/Objects/Kontakt/GroupList/GroupList-001")?;

        GroupList::try_from(&Chunk::read(&mut file)?)?;

        // Ensure the read completed
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        assert_eq!(buf.len(), 0, "Excess data found: {} bytes", buf.len());

        Ok(())
    }
}
