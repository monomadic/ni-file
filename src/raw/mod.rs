/// Raw method
/// - attempts to read files incrementally
/// - safer, better error messages
/// - only parses specific data on demand
///
use crate::prelude::*;
use byteorder::{LittleEndian, ReadBytesExt};

pub struct Frame {
    pub kind: u32,
    pub uuid: [u8; 16],
    pub data: Vec<u8>,
    pub children: Vec<Vec<u8>>,
}

/// assert that a frame has a valid frame size value in its header.
pub fn check_frame_size(bytes: &[u8]) -> Result<()> {
    let buffer: [u8; 4] = bytes.try_into()?;
    let result = u32::from_le_bytes(buffer);
}

pub fn read(buffer: &[u8]) -> Result<()> {
    let size = buffer.read_u64::<LittleEndian>().unwrap();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading_files() -> Result<()> {
        // valid container files
        for entry in glob::glob("data/files/**/*.nki")? {
            let file = std::fs::read(entry?)?;
            let container = read(&file)?;
        }
        Ok(())
    }
}
