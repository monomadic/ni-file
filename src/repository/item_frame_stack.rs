use crate::{prelude::*, read_bytes::ReadBytesExt};

/// A stack of frames
#[derive(Debug, Clone)]
pub struct ItemFrameStack(pub Vec<u8>);

impl ItemFrameStack {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {
        log::debug!("Reading ItemFrameStack");
        let buffer = reader.read_sized_data()?;
        // let mut buf = buffer.clone().as_slice();
        Ok(Self(buffer))
    }
}
