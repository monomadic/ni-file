/* Item
 * NI::SOUND::Container
 *
 */

use crate::read_bytes::ReadBytesExt;
use thiserror::Error;

use super::{header::ItemHeader, item_frame_stack::ItemFrameStack};

/// The basic building block of repositories.
pub struct Item(pub Vec<u8>);

#[derive(Error, Debug)]
pub enum ItemError {
    #[error("Size field mismatch: expected {expected}, got {got}")]
    IncorrectFrameSize { expected: u64, got: u64 },

    #[error("IO Error")]
    IO(#[from] std::io::Error),
}

impl Item {
    /// read a byte stream into a raw Frame
    pub fn read<R>(mut reader: R) -> Result<Item, ItemError>
    where
        R: ReadBytesExt,
    {
        Ok(Item(reader.read_sized_data()?))
    }

    /// read the header data as a byte array
    pub fn header(&self) -> Result<ItemHeader, ItemError> {
        let slice = self.0.as_slice().read_bytes(20)?;
        let frameheader = ItemHeader::read(slice.as_slice())?;
        Ok(frameheader)
    }

    /// read the frame stack as a byte array
    pub fn frame_stack(&self) -> Result<ItemFrameStack, ItemError> {
        let data = self.0.clone();
        let mut data = data.as_slice();
        let _ = data.read_bytes(20)?; // skip header
        let data_frame = ItemFrameStack(data.read_sized_data()?);
        Ok(data_frame)
    }

    /// read the frame stack as a byte array
    pub fn children(&self) -> Result<Vec<Item>, ItemError> {
        let buf = self.0.clone();
        let mut buf = buf.as_slice();

        let _ = buf.read_bytes(40)?; // skip header
        log::debug!("read item header");

        let _ = buf.read_sized_data()?; // skip framestack
        log::debug!("read frame stack");

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
            Ok(children)
        } else {
            // empty vec as there is no more metadata to read
            Ok(vec![])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading_files() -> Result<(), Box<dyn std::error::Error>> {
        crate::utils::setup_logger();

        for path in crate::utils::get_test_files()? {
            log::info!("reading {:?}", path);
            let file = std::fs::read(&path)?;
            let bytes = file.as_slice().read_sized_data()?;

            assert_eq!(bytes.len(), file.len() - 8);
        }
        Ok(())
    }

    #[test]
    fn test_children() -> Result<(), Box<dyn std::error::Error>> {
        crate::utils::setup_logger();

        let item =
            Item(include_bytes!("../../tests/data/files/kontakt-7/000-default.nki").to_vec());

        let children = item.children()?;
        assert_eq!(children.len(), 1);

        Ok(())
    }
}
