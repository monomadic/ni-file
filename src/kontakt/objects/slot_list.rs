use std::{collections::HashMap, io::Cursor};

use crate::{
    kontakt::{structured_object::StructuredObject, Chunk, KontaktError},
    read_bytes::ReadBytesExt,
};

pub const KONTAKT_SLOTLIST_ID: u16 = 0x37;

type Error = crate::NIFileError;

#[derive(Debug)]
pub struct SlotList(StructuredObject);

#[derive(Debug)]
pub struct SlotListPublicParams {
    slots: HashMap<u16, StructuredObject>,
}

impl SlotList {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let so = StructuredObject::read(&mut reader)?;

        Ok(Self(so))
    }

    pub fn params(&self) -> Result<SlotListPublicParams, Error> {
        let mut reader = Cursor::new(&self.0.public_data);

        // Read 8 bytes into fields[0..8]
        let mut slot_flags = Vec::new();
        for _ in 0..8 {
            slot_flags.push(reader.read_u8()?);
        }
        dbg!(&slot_flags);

        // Iterate 64 times
        let mut slots = HashMap::new();
        for i in 0..64 {
            // Check if the corresponding bit in fields[0..8] is set
            if (slot_flags[i >> 3] >> (i & 7) & 1) != 0 {
                // Read chunk data
                let chunk = Chunk::read(&mut reader)?;
                dbg!(&chunk.id);

                // Create StructuredObject
                let obj = StructuredObject::read(&mut Cursor::new(chunk.data))?;
                dbg!(&obj);

                // Add 'obj' to slot
                slots.insert(i as u16, obj);
            }
        }
        Ok(SlotListPublicParams { slots })
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

impl StructuredObject {
    pub fn find_slot_list(&self) -> Result<SlotList, Error> {
        self.find_first(KONTAKT_SLOTLIST_ID)
            .ok_or(KontaktError::MissingChunk(KONTAKT_SLOTLIST_ID).into())
            .and_then(SlotList::try_from)
    }
}
