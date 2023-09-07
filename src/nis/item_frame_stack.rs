use std::io::Cursor;

use crate::{prelude::*, read_bytes::ReadBytesExt};

/// A stack of frames
#[derive(Debug, Clone)]
pub struct ItemFrameStack(pub Cursor<Vec<u8>>);

impl ItemFrameStack {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {
        let len = reader.read_u64_le()? as usize;
        reader.seek(io::SeekFrom::Current(-8))?;
        let cursor = Cursor::new(reader.read_bytes(len)?);
        Ok(Self(cursor))
    }
}
