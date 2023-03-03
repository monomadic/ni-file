/*
 *  trait: ItemReader
 */

use super::{header::ItemHeader, item_frame_stack::ItemFrameStack};
use crate::prelude::*;
use crate::read_bytes::ReadBytesExt;
use thiserror::Error;

// pub trait _ItemReader: io::BufRead {
//     /// read the header data as a byte array
//     fn header(&mut self) -> Result<ItemHeader> {
//         let buf = B
//             self.fill_buf()?;
//         self.consume(40);
//         ItemHeader::read(buf)
//     }
// }

pub trait ItemReader: ReadBytesExt + Sized {
    /// read the header data as a byte array
    fn header(&mut self) -> Result<ItemHeader> {
        let buf = self.by_ref();
        let slice = buf.read_bytes(40)?;
        ItemHeader::read(slice.as_slice())
    }

    fn frame_stack(&mut self) -> Result<ItemFrameStack> {
        log::debug!("ItemReader::frame_stack()");

        // FIXME: the error is here

        // skip header
        // let _ = self.header()?;
        let buf = self.by_ref();
        let _ = buf.read_bytes(40)?;
        // std::io::copy(&mut buf.by_ref().take(27), &mut std::io::sink());

        // read frame stack into a buffer
        ItemFrameStack::read(buf.read_sized_data()?.as_slice())
    }

    // fn frame(&mut self) -> Result<ItemFrame> {
    //     // skip header
    //     // let _ = self.header()?;
    //     let _ = self.read_bytes(40)?;
    //     // read frame stack into a buffer
    //     let buf = self.read_sized_data()?;
    //     let header = ItemFrameHeader::read(buf.as_slice())?;
    //
    //     println!("ItemFrameHeader {:?}", &header);
    //
    //     log::debug!("ItemID found: {:?}", ItemID::from(header.item_id));
    //
    //     Ok(match ItemID::from(header.item_id) {
    //         ItemID::RepositoryRoot => ItemFrame::RepositoryRoot(RepositoryRoot::read(self)?),
    //         ItemID::BNISoundPreset => ItemFrame::BNISoundPreset(BNISoundPreset::read(self)?),
    //         _ => todo!(),
    //     })
    // }

    /// read the frame stack as a byte array
    fn children(&mut self) -> Result<Vec<Item>> {
        let buf = self.by_ref();
        let _ = buf.read_bytes(40)?; // skip header
        log::debug!("read item header");
        Ok(vec![])
    }
}

/// The basic building block of repositories.
pub struct Item(Vec<u8>);

#[derive(Error, Debug)]
pub enum ItemError {
    #[error("Size field mismatch: expected {expected}, got {got}")]
    IncorrectFrameSize { expected: u64, got: u64 },

    #[error("IO Error")]
    IO(#[from] std::io::Error),
}

impl Item {
    /// read a byte stream into a raw Frame
    pub fn read<R>(mut reader: R) -> Result<Self>
    where
        R: ReadBytesExt,
    {
        Ok(Item(reader.read_sized_data()?))
    }

    /// read the header data as a byte array
    pub fn header(&self) -> Result<ItemHeader> {
        let slice = self.0.as_slice().read_bytes(20)?;
        let frameheader = ItemHeader::read(slice.as_slice())?;
        Ok(frameheader)
    }

    /// read the frame stack as a byte array
    pub fn frame_stack(&self) -> Result<ItemFrameStack> {
        // let mut data = self.0.as_slice();
        let mut data = io::BufReader::new(self.0.as_slice());
        // let mut data = self.0;
        let _header = ItemHeader::read(&mut data)?;
        let frame = ItemFrameStack::read(&mut data)?;
        Ok(frame)
    }

    /// read the frame stack as a byte array
    pub fn children(&self) -> Result<Vec<Item>> {
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
    fn test_item_read() -> Result<()> {
        crate::utils::setup_logger();

        println!("{:?}", crate::utils::get_files("tests/data/files/**/*.*")?);

        for path in crate::utils::get_test_files()? {
            log::info!("reading {:?}", path);

            let file = std::fs::read(&path)?;
            let item: Item = Item::read(file.as_slice())?;

            assert_eq!(item.0.len(), file.len());
        }

        Ok(())
    }

    #[test]
    fn test_item_frame_stack() -> Result<()> {
        crate::utils::setup_logger();

        for path in crate::utils::get_test_files()? {
            log::info!("reading {:?}", path);

            let file = std::fs::read(&path)?;
            let item: Item = Item::read(file.as_slice())?;

            let stack = item.frame_stack()?;
            // assert!(stack.item().is_ok());
        }

        Ok(())
    }

    #[test]
    fn test_children() -> Result<()> {
        crate::utils::setup_logger();

        let item =
            Item(include_bytes!("../../tests/data/files/kontakt-7/000-default.nki").to_vec());
        let children = item.children()?;

        assert_eq!(children.len(), 1);
        Ok(())
    }
}
