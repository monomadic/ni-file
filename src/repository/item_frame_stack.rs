use crate::prelude::*;
use crate::read_bytes::ReadBytesExt;

use super::item_frame::repository_root::RepositoryRoot;

/// A stack of frames
pub struct ItemFrameStack(pub Vec<u8>);

impl ItemFrameStack {
    pub fn read<R>(mut reader: R) -> Result<Self>
    where
        R: ReadBytesExt,
    {
        Ok(Self(reader.read_sized_data()?))
    }

    pub fn pop(&mut self) -> Result<ItemFrame> {
        Ok(ItemFrame::RepositoryRoot(RepositoryRoot::read(
            self.0.as_slice(),
        )?))
    }
}

#[derive(Debug)]
pub enum ItemFrame {
    RepositoryRoot(RepositoryRoot),
}
