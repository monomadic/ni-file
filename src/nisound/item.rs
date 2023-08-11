use super::{
    header::ItemHeader,
    item_frame::{item_id::ItemID, ItemFrame},
};
use crate::{prelude::*, read_bytes::ReadBytesExt};

/// NISound documents are made up of nested [`Item`]s.
#[derive(Clone, Debug)]
pub struct ItemContainer {
    pub header: ItemHeader,
    pub data: Vec<u8>,
    pub children: Vec<ItemContainer>,
}

impl ItemContainer {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {
        log::debug!("RepositoryRoot::read");

        let buffer = reader.read_sized_data()?;
        let mut buffer = buffer.as_slice();

        Ok(ItemContainer {
            header: ItemHeader::read(&mut buffer)?,
            // data: ItemFrameStack::read(&mut buffer)?,
            data: buffer.read_sized_data()?,
            children: ItemContainer::read_children(&mut buffer)?,
        })
    }

    pub fn data(&self) -> Result<ItemFrame> {
        ItemFrame::read(self.data.as_slice())
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

                children.push(ItemContainer::read(buf.read_sized_data()?.as_slice())?);
            }
        }
        Ok(children)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::get_files;

    #[test]
    fn test_item_read() -> Result<()> {
        for path in get_files("tests/data/nisound/file/**/*.*")? {
            log::info!("reading {:?}", path);

            let file = std::fs::File::open(&path)?;
            ItemContainer::read(file)?;
        }

        Ok(())
    }

    #[test]
    fn test_item_frame() -> Result<()> {
        for path in get_files("tests/data/nisound/file/**/*.*")? {
            log::info!("reading {:?}", path);

            let file = std::fs::File::open(&path)?;
            let _item: ItemContainer = ItemContainer::read(file)?;
        }

        Ok(())
    }

    #[test]
    fn test_children() -> Result<()> {
        let data = include_bytes!("../../tests/filetype/NISD/kontakt/7.1.3.0/000-default.nki");
        let mut data = data.as_slice();

        let item = ItemContainer::read(&mut data)?;
        let children = item.children;

        assert_eq!(children.len(), 1);
        Ok(())
    }
}
