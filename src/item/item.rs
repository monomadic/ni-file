use crate::read_bytes::ReadBytesExt;
use thiserror::Error;

use super::{frame_stack::ItemFrameStack, header::ItemHeader};

/// named type for a raw frame
pub struct Item(pub Vec<u8>);

#[derive(Error, Debug)]
pub enum FrameError {
    #[error("Size field mismatch: expected {expected}, got {got}")]
    IncorrectFrameSize { expected: u64, got: u64 },

    #[error("IO Error")]
    IO(#[from] std::io::Error),
}

impl Item {
    /// read a byte stream into a raw Frame
    pub fn read<R>(mut reader: R) -> Result<Item, FrameError>
    where
        R: ReadBytesExt,
    {
        Ok(Item(reader.read_sized_data()?))
    }

    /// read the header data as a byte array
    pub fn header(&self) -> Result<ItemHeader, FrameError> {
        let slice = self.0.as_slice().read_bytes(20)?;
        let frameheader = ItemHeader::read(slice.as_slice())?;
        Ok(frameheader)
    }

    /// read the frame stack as a byte array
    pub fn frame_stack(&self) -> Result<ItemFrameStack, FrameError> {
        let data = self.0.clone();
        let mut data = data.as_slice();
        let _ = data.read_bytes(20)?; // header
        let data_frame = ItemFrameStack(data.read_sized_data()?);
        Ok(data_frame)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_test() -> Result<(), Box<dyn std::error::Error>> {
        let bytes = [12_u64.to_le_bytes().to_vec(), 64_u32.to_le_bytes().to_vec()].concat();
        assert_eq!(bytes.as_slice().read_sized_data()?, 64_u32.to_le_bytes());
        Ok(())
    }

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
}