use std::io::Cursor;

use crate::{prelude::*, read_bytes::ReadBytesExt};

use super::{ItemData, ItemHeader, ItemType};

/// NISound documents are made up of nested [`Item`]s.
#[derive(Clone, Debug)]
pub struct ItemContainer {
    pub header: ItemHeader,
    pub data: ItemData,
    pub children: Vec<ItemContainer>,
}

impl ItemContainer {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {
        let header = ItemHeader::read(&mut reader)?;
        let length = header.length - 40;
        let mut chunk_data = Cursor::new(reader.read_bytes(length as usize)?);

        Ok(ItemContainer {
            header,
            data: ItemData::read(&mut chunk_data)?,
            children: ItemContainer::read_children(&mut chunk_data)?,
        })
    }

    pub fn first_child(&self) -> Option<&ItemContainer> {
        self.children.get(0)
    }

    pub fn id(&self) -> ItemType {
        self.data.header.item_type()
    }

    /// Returns the first instance of Item by ItemID within child Items.
    pub fn find(&self, kind: &ItemType) -> Option<&ItemContainer> {
        // Check this Item first
        if &self.data.header.item_type() == kind {
            return Some(&self);
        }
        // Recursively search the children
        for item in &self.children {
            if let Some(frame) = item.find(kind) {
                return Some(frame);
            }
        }
        None
    }

    /// Returns the first instance of Item by ItemID within child Items.
    pub fn find_data(&self, kind: &ItemType) -> Option<&ItemData> {
        // Check this Item first
        if &self.data.header.item_type() == kind {
            return Some(&self.data);
        }
        // Recursively search the children
        for item in &self.children {
            if let Some(frame) = item.find_data(kind) {
                return Some(frame);
            }
        }
        None
    }

    /// Find the first Item of type ItemID in the document and return it
    pub fn find_item<'a, I>(&'a self, kind: &'a ItemType) -> Option<Result<I>>
    where
        I: TryFrom<&'a ItemData, Error = NIFileError>,
    {
        self.find_data(&kind).map(I::try_from)
    }

    fn read_children<R: ReadBytesExt>(mut buf: R) -> Result<Vec<ItemContainer>> {
        let version = buf.read_u32_le()?;
        debug_assert_eq!(version, 1);

        let num_children = buf.read_u32_le()?;

        let mut children = Vec::new();
        if num_children > 0 {
            for _ in 0..num_children {
                // note: siblingIndex for soundinfoitem is 1001 to ensure it is last
                let _index = buf.read_u32_le()?;

                // childs domain id
                let _domain_id = buf.read_u32_le()?;
                let _item_id = buf.read_u32_le()?;

                // let pos = buf.stream_position()?;
                // let len = buf.read_u64_le()? as usize;
                // buf.seek(io::SeekFrom::Start(pos))?;

                let len = buf.read_u64_le()? as usize;
                buf.seek(io::SeekFrom::Current(-8))?;

                let data = Cursor::new(buf.read_bytes(len)?);

                children.push(ItemContainer::read(data)?);
            }
        }
        Ok(children)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_item_read() -> Result<()> {
        let data = File::open("test-data/NIS/Item/BNISoundPreset/BNISoundPreset-000")?;
        let item = ItemContainer::read(data)?;
        assert_eq!(item.children.len(), 0);
        Ok(())
    }

    #[test]
    fn test_item_with_children_read() -> Result<()> {
        let data = File::open("tests/filetype/NISD/kontakt/7.1.3.0/000-default.nki")?;
        let item = ItemContainer::read(data)?;
        assert_eq!(item.children.len(), 1);
        Ok(())
    }
}
