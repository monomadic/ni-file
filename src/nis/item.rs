use std::io::Cursor;

use super::{item_data::ItemData, item_header::ItemHeader, ItemID};
use crate::{prelude::*, read_bytes::ReadBytesExt};

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

    /// Returns the first instance of Item by ItemID within child Items.
    pub fn find(&self, kind: &ItemID) -> Option<&ItemData> {
        // Check this Item first
        if &self.data.header.item_id == kind {
            return Some(&self.data);
        }
        // Recursively search the children
        for item in &self.children {
            if let Some(frame) = item.find(kind) {
                return Some(frame);
            }
        }
        None
    }

    fn read_children<R: ReadBytesExt>(mut buf: R) -> Result<Vec<ItemContainer>> {
        let version = buf.read_u32_le()?;
        debug_assert_eq!(version, 1);

        let num_children = buf.read_u32_le()?;

        let mut children = Vec::new();
        if num_children > 0 {
            for _ in 0..num_children {
                let _unknown = buf.read_u32_le()?;

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
