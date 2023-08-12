/*
    SubtreeItem (0x73, 115)
    appears on compressed segments

    u32  1
    bool ?
    u32  decompressed_size
    u32  compressed_size
    &[compressed_size;u8] compressed_data

    SubtreeItem.readItem(&stream) {
        let header_item = Item::readItem(&stream)?;

        if stream.read_u32 != 0 {
            return Err(VERSION_MISMATCH);
        }

        let is_compressed = stream.read_bool();
        header_item[6] = is_compressed;

        if !is_compressed {
            eax_12 = Item::read();
            return;
        }

        let decompressed_size = stream.read_u32();
        let compressed_size = stream.read_u32();

        let mut buffer;
        let size = stream.read_raw(&buffer, compressed_size);

        if size != compressed_size {
            return Err(INTERNAL_ERROR);
        }

        if is_compressed {
            return SubtreeItem::decompressInputStream(&stream);
        }
    }
*/

use std::io::Cursor;

use crate::nisound::item_frame::ItemFrame;
use crate::nisound::ItemID;
use crate::prelude::*;
use crate::read_bytes::ReadBytesExt;

pub struct SubtreeItem {
    pub inner_data: Vec<u8>,
}

impl std::convert::TryFrom<ItemFrame> for SubtreeItem {
    type Error = NIFileError;

    fn try_from(frame: ItemFrame) -> Result<Self> {
        debug_assert_eq!(frame.header.item_id, ItemID::SubtreeItem);
        Self::read(Cursor::new(frame.data))
    }
}

impl SubtreeItem {
    /// decompress and return compressed internal Item.
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {
        let prop_version = reader.read_u32_le()?;
        assert_eq!(prop_version, 1);

        let is_compressed = reader.read_u8()?;
        log::debug!("is_compressed: {}", is_compressed);

        let decompressed_size = reader.read_u32_le()?;
        log::debug!("decompressed_size: {}", decompressed_size);

        let compressed_size = reader.read_u32_le()?;
        log::debug!("compressed_size: {}", compressed_size);

        let compressed_data = reader.read_bytes(compressed_size as usize)?;

        // let inner_data =
        //     crate::deflate::deflate_checked(&compressed_data, decompressed_size as usize)?;

        let inner_data =
            crate::deflate::deflate_with_lib(&compressed_data, decompressed_size as usize)?;

        Ok(SubtreeItem { inner_data })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_subtree() -> Result<()> {
        let data = Cursor::new(include_bytes!(
            "../../../tests/data/nisound/chunks/item-frame-property/kontakt-4/115-SubtreeItem.data"
        ));
        let item = SubtreeItem::read(data)?;

        assert_eq!(item.inner_data.len(), 4524);

        Ok(())
    }
}
