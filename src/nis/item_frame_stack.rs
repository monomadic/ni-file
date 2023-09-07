use std::io::Cursor;

use crate::{prelude::*, read_bytes::ReadBytesExt};

/// A stack of frames
#[derive(Debug, Clone)]
pub struct ItemFrameStack(pub Cursor<Vec<u8>>);

impl ItemFrameStack {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {
        // FIXME: expects a rewind
        let len = reader.read_u64_le()? as usize;
        let cursor = Cursor::new(reader.read_bytes(len)?);
        Ok(Self(cursor))
    }
}
