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

// TODO: should work on frames, not raw property data

use crate::prelude::*;
use crate::read_bytes::ReadBytesExt;

pub struct SubtreeItem(Vec<u8>);

impl SubtreeItem {
    /// decompress and return compressed internal Item.
    fn read(&self) -> Result<SubtreeItem, NIFileError> {
        let mut buf = self.0.as_slice();

        let prop_version = buf.read_u32_le()?;
        debug_assert_eq!(prop_version, 1);

        let is_compressed = buf.read_u8()?;
        log::debug!("is_compressed: {}", is_compressed);

        let decompressed_size = buf.read_u32_le()?;
        log::debug!("decompressed_size: {}", decompressed_size);

        let compressed_size = buf.read_u32_le()?;
        log::debug!("compressed_size: {}", compressed_size);

        let compressed_data = buf.read_bytes(compressed_size as usize)?;

        let inner_data =
            crate::decompress::decompress(&compressed_data, decompressed_size as usize).unwrap();
        // debug_assert!();

        Ok(SubtreeItem(inner_data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_read_subtree() -> Result<(), Box<dyn Error>> {
        crate::utils::setup_logger();

        let data = include_bytes!(
            "../../../tests/data/item-frame-property/kontakt-4/115-SubtreeItem.data"
        );
        let item = SubtreeItem(data.to_vec());
        let inner = item.read()?;

        assert_eq!(inner.0.len(), 4524);

        Ok(())
    }
}
