// SubtreeItem
//
// Properties
// - num-hidden-items

/*
    SubtreeItem (0x73, 115)
    appears on compressed segments

    u32  1
    bool is_compressed
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

use crate::nis::{ItemContainer, ItemData, ItemType};
use crate::prelude::*;
use crate::read_bytes::ReadBytesExt;

#[derive(Debug)]
pub struct SubtreeItem {
    pub inner_data: Vec<u8>,
}

impl std::convert::TryFrom<&ItemData> for SubtreeItem {
    type Error = NIFileError;

    fn try_from(frame: &ItemData) -> Result<Self> {
        debug_assert_eq!(frame.header.item_type(), ItemType::SubtreeItem);
        Self::read(Cursor::new(&frame.data))
    }
}

impl SubtreeItem {
    /// Decompress and return compressed internal Item.
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {
        let prop_version = reader.read_u32_le()?; // num items?
        assert_eq!(prop_version, 1);

        let is_compressed = reader.read_bool()?;
        let inner_data = match is_compressed {
            true => {
                let decompressed_size = reader.read_u32_le()? as usize;
                let compressed_size = reader.read_u32_le()?;
                let compressed_data = reader.read_bytes(compressed_size as usize)?;

                let output = &mut vec![0_u8; decompressed_size];
                fastlz::decompress(&compressed_data, output)
                    .map_err(|_| NIFileError::Generic("lz77".into()))?
                    .to_vec()

                // lz77::decompress(&mut Cursor::new(compressed_data))
                //     .map_err(|e| NIFileError::Generic(e.to_string()))?
            }
            false => {
                let decompressed_size = reader.read_u64_le()? as usize;
                reader.seek(io::SeekFrom::Current(-8))?;
                reader.read_bytes(decompressed_size as usize)?
            }
        };

        Ok(SubtreeItem { inner_data })
    }

    pub fn item(&self) -> Result<ItemContainer> {
        let container = ItemContainer::read(Cursor::new(&self.inner_data))?;
        Ok(container)
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use super::*;

    #[test]
    fn test_read_subtree() -> Result<()> {
        let mut data = File::open("tests/data/Containers/NIS/objects/SubtreeItem/SubtreeItem-000")?;
        let subtree = SubtreeItem::read(&mut data)?;

        assert_eq!(subtree.inner_data.len(), 4524);
        let item = subtree.item()?;

        assert_eq!(item.id(), ItemType::Item);

        // ensure the read completed
        let mut buf = Vec::new();
        data.read_to_end(&mut buf)?;
        assert_eq!(buf.len(), 0, "Excess data found");

        Ok(())
    }
}
