pub mod app_id;
pub mod domain_id;
pub mod item_frame_header;
pub mod item_id;

pub use item_frame_header::ItemFrameHeader;

use super::item_frame_stack::ItemFrameStack;
use crate::{prelude::*, read_bytes::ReadBytesExt};
use item_id::ItemID;
use std::convert::TryFrom;

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
        let header = ItemFrameHeader::read(&mut reader)?;
        let inner = ItemFrameStack::read(&mut reader)?;
        let data = reader.read_bytes((header.length as usize - 20 - inner.0.len()) as usize)?;

        if header.item_id == ItemID::Item {
            return Err(NIFileError::ItemTerminator);
        }

        Ok(Self {
            header,
            inner,
            data,
        })
    }

    pub fn inner(&self) -> Option<ItemFrame> {
        ItemFrame::try_from(&self.inner).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_frame_read() -> Result<()> {
        let file = include_bytes!("../../../tests/patchdata/NISD/ItemFrame/RepositoryRoot-000");
        let item = ItemFrame::read(file.as_slice())?;
        assert_eq!(item.data.len(), 58);

        assert_eq!(item.header.item_id, ItemID::RepositoryRoot);
        assert_eq!(item.inner().unwrap().header.item_id, ItemID::Authorization);

        Ok(())
    }
}
