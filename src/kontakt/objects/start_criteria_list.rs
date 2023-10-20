use std::io::Cursor;

use crate::{
    kontakt::{objects::StartCriteriaParams, Chunk, KontaktError},
    read_bytes::ReadBytesExt,
    Error,
};

const CHUNK_ID: u16 = 0x38;

/// StartCriteriaList
///
/// Group start options - determines conditions for which a group
/// is triggered. Maximum of 4 conditions per group.
///
/// Type:           Chunk<Raw>
/// SerType:        ?
/// Kontakt 7:      ?
/// KontaktIO:      StartCritList
#[derive(Debug)]
pub struct StartCriteriaList {
    pub items: Vec<StartCriteriaParams>,
}

impl StartCriteriaList {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let num_items = reader.read_i8()?;
        let mut items = Vec::new();

        assert!(num_items <= 4);

        for i in 0..num_items {
            if num_items & (1 << (i & 0x1F)) != 0 {
                // ensure raw data
                let is_structured_object = reader.read_bool()?;
                assert!(!is_structured_object);

                // ensure startcriteria v70
                let version = reader.read_u16_le()?;
                assert_eq!(version, 0x70);

                let item = StartCriteriaParams::read(&mut reader)?;
                items.push(item);
            }
        }

        Ok(Self { items })
    }
}

impl std::convert::TryFrom<&Chunk> for StartCriteriaList {
    type Error = Error;

    fn try_from(chunk: &Chunk) -> Result<Self, Self::Error> {
        if chunk.id != CHUNK_ID {
            return Err(KontaktError::IncorrectID {
                expected: CHUNK_ID,
                got: chunk.id,
            }
            .into());
        }
        let reader = Cursor::new(&chunk.data);
        Self::read(reader)
    }
}
