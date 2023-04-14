pub mod app_id;
pub mod domain_id;
pub mod item_frame_header;
pub mod item_id;

use std::convert::TryFrom;

pub use item_frame_header::ItemFrameHeader;

use crate::{prelude::*, read_bytes::ReadBytesExt};

use super::item_frame_stack::ItemFrameStack;

#[derive(Clone, Debug)]
pub struct ItemFrame {
    pub header: ItemFrameHeader,
    pub inner: ItemFrameStack,
    pub data: Vec<u8>,
}

impl std::convert::TryFrom<&ItemFrameStack> for ItemFrame {
    type Error = NIFileError;

    fn try_from(stack: &ItemFrameStack) -> Result<Self> {
        ItemFrame::read(stack.0.as_slice())
    }
}

impl ItemFrame {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {
        log::debug!("ItemFrame::read");

        let buf = reader.read_sized_data()?;
        let mut buf = buf.as_slice();
        let header = ItemFrameHeader::read(&mut buf)?;

        if header.item_id == crate::ItemID::Item {
            panic!("reading terminator frame");
        }

        let inner = ItemFrameStack::read(&mut buf)?;

        Ok(Self {
            header,
            inner,
            data: buf.to_vec(),
        })
    }

    pub fn inner(&self) -> Result<ItemFrame> {
        ItemFrame::try_from(&self.inner)
    }
}
