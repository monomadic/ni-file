pub mod app_id;
pub mod domain_id;
pub mod item_frame_header;
pub mod item_id;

// frame data
pub mod authorization;
pub mod bni_sound_preset;
pub mod encryption_item;
pub mod preset;
pub mod repository_root;
pub mod sound_info;
pub mod sound_info_item;
pub mod subtree_item;

pub use item_frame_header::ItemFrameHeader;

use crate::{prelude::*, read_bytes::ReadBytesExt};

#[derive(Clone, Debug)]
pub struct ItemFrame {
    pub header: ItemFrameHeader,
    pub inner: Vec<u8>,
    pub data: Vec<u8>,
}

impl std::convert::TryFrom<Vec<u8>> for ItemFrame {
    type Error = NIFileError;

    fn try_from(buf: Vec<u8>) -> Result<Self> {
        ItemFrame::read(buf.as_slice())
    }
}

impl ItemFrame {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {
        log::debug!("ItemFrame::read");

        let buf = reader.read_sized_data()?;
        let mut buf = buf.as_slice();
        let header = ItemFrameHeader::read(&mut buf)?;
        let inner = buf.read_sized_data()?;

        Ok(Self {
            header,
            inner,
            data: buf.to_vec(),
        })
    }

    pub fn frame(&self) -> Result<Self> {
        Self::read(self.inner.as_slice())
    }
}
