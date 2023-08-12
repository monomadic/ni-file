use std::io::Cursor;

use super::{
    header::ItemHeader,
    item_frame::{item_id::ItemID, ItemFrame},
};
use crate::{prelude::*, read_bytes::ReadBytesExt};

/// NISound documents are made up of nested [`Item`]s.
#[derive(Clone, Debug)]
pub struct ItemContainer {
    pub header: ItemHeader,
    pub items: ItemFrame,
    pub children: Vec<ItemContainer>,
}

impl ItemContainer {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {
        let header = ItemHeader::read(&mut reader)?;
        let length = header.length - 40;
        let mut chunk_data = Cursor::new(reader.read_bytes(length as usize)?);

        Ok(ItemContainer {
            header,
            items: ItemFrame::read(&mut chunk_data)?,
            children: Vec::new(), //ItemContainer::read_children(&mut reader)?,
        })
    }

    /// Returns the first instance of Item by ItemID within child Items.
    pub fn find(&self, kind: &ItemID) -> Option<&ItemFrame> {
        // check this Item first
        if &self.items.header.item_id == kind {
            return Some(&self.items);
        }
        for item in &self.children {
            if let Some(frame) = item.find(kind) {
                return Some(frame);
            }
        }
        None
    }

    fn read_children<R: ReadBytesExt>(mut buf: R) -> Result<Vec<ItemContainer>> {
        log::debug!("RepositoryRoot::read_children");

        let version = buf.read_u32_le()?;
        debug_assert_eq!(version, 1);

        let num_children = buf.read_u32_le()?;
        log::debug!("num_children: {}", num_children);
        // note: need to switch this out as it doesn't work like this

        let mut children = Vec::new();
        if num_children > 0 {
            for _ in 0..num_children {
                let unknown = buf.read_u32_le()?;
                log::debug!("unknown tag: {}", unknown);

                // childs domain id
                let domain_id = buf.read_u32_le()?;
                let item_id = buf.read_u32_le()?;

                log::debug!("child domain_id: {}, item_id: {}", domain_id, item_id);

                let data = Cursor::new(buf.read_sized_data()?);

                children.push(ItemContainer::read(data)?);
            }
        }
        Ok(children)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_read() -> Result<()> {
        let data = std::io::Cursor::new(include_bytes!(
            "../../tests/patchdata/NISD/ItemContainer/ItemContainer-BNISoundPreset-000"
        ));
        ItemContainer::read(data)?;

        Ok(())
    }

    // #[test]
    // fn test_children() -> Result<()> {
    //     let data = include_bytes!("../../tests/filetype/NISD/kontakt/7.1.3.0/000-default.nki");
    //     let mut data = data.as_slice();
    //
    //     let item = ItemContainer::read(&mut data)?;
    //     let children = item.children;
    //
    //     assert_eq!(children.len(), 1);
    //     Ok(())
    // }
}
