use std::{collections::HashMap, io::Cursor};

use crate::{
    kontakt::{structured_object::StructuredObject, Chunk, KontaktError},
    read_bytes::ReadBytesExt,
};

use super::ProgramContainer;

pub const KONTAKT_SLOTLIST_ID: u16 = 0x37;

type Error = crate::NIFileError;

/// Type:           Chunk
/// SerType:        0x37
/// Known Versions:
/// Kontakt 7:      BBank::readSlotList()
/// KontaktIO:      SlotList
#[derive(Debug)]
pub struct SlotList {
    pub slots: HashMap<u16, ProgramContainer>,
}

impl SlotList {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let mut slot_flags = Vec::new();
        let mut slots = HashMap::new();

        // Read 8 bytes into fields[0..8]
        for _ in 0..8 {
            slot_flags.push(reader.read_u8()?);
        }

        // Iterate 64 times
        for i in 0..64 {
            // Check if the corresponding bit in fields[0..8] is set
            if (slot_flags[i >> 3] >> (i & 7) & 1) != 0 {
                dbg!("Read");
                // Read chunk data
                let chunk = Chunk::read(&mut reader)?;
                dbg!(&chunk.id);

                // Add 'obj' to slot
                slots.insert(i as u16, (&chunk).try_into()?);
            }
        }
        Ok(Self { slots })
    }
}

impl std::convert::TryFrom<&Chunk> for SlotList {
    type Error = Error;

    fn try_from(chunk: &Chunk) -> Result<Self, Self::Error> {
        if chunk.id != KONTAKT_SLOTLIST_ID {
            return Err(KontaktError::IncorrectID {
                expected: KONTAKT_SLOTLIST_ID,
                got: chunk.id,
            }
            .into());
        }
        let reader = Cursor::new(&chunk.data);
        Self::read(reader)
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
            "tests/data/Objects/Kontakt/SlotList/SlotList-000.kon",
        )?)?;
        let slotlist = SlotList::try_from(&chunk)?;
        for (i, so) in slotlist.slots {

            // let filename = format!("{:?}-{:x}.chunk", chunk.into_type()?, chunk.id);
            // std::fs::write(filename, chunk.data)?;
        }
        Ok(())
    }
}
