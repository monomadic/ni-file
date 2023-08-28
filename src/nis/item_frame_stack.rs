use std::io::Cursor;

use crate::{prelude::*, read_bytes::ReadBytesExt};

/// A stack of frames
#[derive(Debug, Clone)]
pub struct ItemFrameStack(pub Cursor<Vec<u8>>);

impl ItemFrameStack {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {
        log::debug!("Reading ItemFrameStack");
        let buffer = Cursor::new(reader.read_sized_data()?);
        Ok(Self(buffer))
    }
}
