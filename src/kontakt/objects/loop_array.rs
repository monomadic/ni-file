use std::io::Cursor;

use crate::{
    kontakt::{objects::Loop, Chunk, KontaktError, StructuredObject},
    read_bytes::ReadBytesExt,
    Error,
};

/// Type:           Chunk
/// SerType:        0x39
/// Kontakt 7:      array<BLoop>
/// KontaktIO:      LoopArray
#[derive(Debug)]
pub struct LoopArray {
    pub items: Vec<Loop>,
}

impl LoopArray {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let num_items = reader.read_u8()?;
        let mut items = Vec::new();

        for _ in 0..num_items {
            let so = StructuredObject::read(&mut reader)?;
            assert_eq!(so.children.len(), 0, "Loop objects should have no children");
            let mut reader = Cursor::new(&so.public_data);
            items.push(Loop::read(&mut reader)?);
        }

        Ok(Self { items })
    }
}

impl std::convert::TryFrom<&Chunk> for LoopArray {
    type Error = Error;

    fn try_from(chunk: &Chunk) -> Result<Self, Self::Error> {
        if chunk.id != 0x39 {
            return Err(KontaktError::IncorrectID {
                expected: 0x39,
                got: chunk.id,
            }
            .into());
        }
        let reader = Cursor::new(&chunk.data);
        Self::read(reader)
    }
}
