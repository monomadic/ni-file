use crate::prelude::*;
use crate::read_bytes::ReadBytesExt;

use super::item_frame::{
    bni_sound_preset::BNISoundPreset, item_id::ItemID, repository_root::RepositoryRoot,
    ItemFrameHeader,
};

/// A stack of frames
pub struct ItemFrameStack(pub Vec<u8>);

impl ItemFrameStack {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {
        Ok(Self(reader.read_sized_data()?))
    }

    pub fn frame(&mut self) -> Result<ItemFrame> {
        let buffer = self.0.as_slice();
        let header = ItemFrameHeader::read(buffer)?;

        log::debug!("ItemID found: {:?}", ItemID::from(header.item_id));

        Ok(match ItemID::from(header.item_id) {
            ItemID::RepositoryRoot => ItemFrame::RepositoryRoot(RepositoryRoot::read(buffer)?),
            ItemID::BNISoundPreset => ItemFrame::BNISoundPreset(BNISoundPreset::read(buffer)?),
            _ => todo!(),
        })
    }
}

#[derive(Debug)]
pub enum ItemFrame {
    RepositoryRoot(RepositoryRoot),
    BNISoundPreset(BNISoundPreset),
}
