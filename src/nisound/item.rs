use super::{
    header::ItemHeader,
    item_frame::{item_id::ItemID, ItemFrame},
    item_frame_stack::ItemFrameStack,
};
use crate::{prelude::*, read_bytes::ReadBytesExt};
use std::convert::TryFrom;

/// NISound documents are made up of nested [`Item`]s.
#[derive(Clone, Debug)]
pub struct Item {
    pub header: ItemHeader,
    pub data: ItemFrameStack,
    pub children: Vec<Item>,
}

impl Item {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {
        log::debug!("RepositoryRoot::read");

        let buffer = reader.read_sized_data()?;
        let mut buffer = buffer.as_slice();

        Ok(Item {
            header: ItemHeader::read(&mut buffer)?,
            data: ItemFrameStack::read(&mut buffer)?,
            children: Item::read_children(&mut buffer)?,
        })
    }

    pub fn data(&self) -> Result<ItemFrame> {
        ItemFrame::try_from(&self.data)
    }

    /// Returns the first instance of Item by ItemID within child Items.
    pub fn find(&self, kind: &ItemID) -> Option<ItemFrame> {
        // check this Item first
        if let Some(frame) = self.data().ok() {
            if &frame.header.item_id == kind {
                return Some(frame);
            }
            for item in &self.children {
                if let Some(frame) = item.find(kind) {
                    return Some(frame);
                }
            }
        }
        None
    }

    fn read_children<R: ReadBytesExt>(mut buf: R) -> Result<Vec<Item>> {
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

                // There is a wasteful 12 bytes per child here telling the code how to read the next
                // segment. This should not be necessary as you could read the child generically but
                // could have been a limitation of the original language or codebase.

                let domain_id = buf.read_u32_le()?;
                let item_id = buf.read_u32_le()?;
                log::debug!("child domain_id: {}, item_id: {}", domain_id, item_id);

                children.push(Item::read(buf.read_sized_data()?.as_slice())?);
            }
        }
        Ok(children)
    }

    // fn find_item(item_id: ItemID) -> ItemData {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::get_files;

    #[test]
    fn test_item_read() -> Result<()> {
        // crate::utils::setup_logger();

        for path in get_files("tests/data/files/**/*.*")? {
            log::info!("reading {:?}", path);

            let file = std::fs::File::open(&path)?;
            Item::read(file)?;
        }

        Ok(())
    }

    #[test]
    fn test_item_frame() -> Result<()> {
        //crate::utils::setup_logger();

        for path in get_files("tests/data/files/**/*.*")? {
            log::info!("reading {:?}", path);

            let file = std::fs::File::open(&path)?;
            let _item: Item = Item::read(file)?;
        }

        Ok(())
    }

    #[test]
    fn test_children() -> Result<()> {
        // crate::utils::setup_logger();

        let data = include_bytes!("../../tests/data/nisound/file/kontakt/7.1.3.0/000-default.nki");
        let mut data = data.as_slice();

        let item = Item::read(&mut data)?;
        let children = item.children;

        assert_eq!(children.len(), 1);
        Ok(())
    }
}
