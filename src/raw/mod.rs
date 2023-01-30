use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Read;

/// Raw method
/// - attempts to read files incrementally
/// - safer, better error messages
/// - only parses specific data on demand
///
use thiserror::Error;

pub struct Frame {
    pub kind: u32,
    pub uuid: [u8; 16],
    pub data: Vec<u8>,
    pub children: Vec<Vec<u8>>,
}

fn read_frame_chunk<'a>(mut buffer: &'a [u8]) -> Result<&'a [u8], FrameError> {
    // read size field
    let buffer_size = buffer.len() as u64;
    log::debug!("Received buffer size: {}", buffer_size);

    let size_field = buffer.read_u64::<LittleEndian>()?;
    log::debug!("Read size field: {}", size_field);

    if buffer_size != size_field {
        return Err(FrameError::IncorrectFrameSize {
            expected: size_field,
            got: buffer_size,
        });
    }

    // TODO: read buffer_size bytes
    let bytes_to_read: usize = (buffer_size - size_field) as usize;
    let mut buf = vec![0u8; bytes_to_read];
    buffer.read_exact(&mut buf)?;

    Ok(&buffer)
}

/* fn consume_u32_le(bytes: &mut [u8]) -> Result<u32, std::array::TryFromSliceError> {
    let (u32_buffer, _) = buffer.split_at_mut(std::mem::size_of::<u32>());
    u32::from_le_bytes(u32_buffer)
} */

#[derive(Error, Debug)]
pub enum FrameError {
    #[error("Incorrect Size Field: expected {expected}, got {got}")]
    IncorrectFrameSize { expected: u64, got: u64 },

    #[error("IO Error")]
    IO(#[from] std::io::Error),
}

impl Frame {
    pub fn read(mut buffer: &[u8]) -> Result<(), FrameError> {
        // read size field
        let buffer_size = buffer.len() as u64;
        log::debug!("Received buffer size: {}", buffer_size);

        let size_field = buffer.read_u64::<LittleEndian>()?;
        log::debug!("Read size field: {}", size_field);

        if buffer_size != size_field {
            return Err(FrameError::IncorrectFrameSize {
                expected: size_field,
                got: buffer_size,
            });
        }

        // TODO: return errors instead of asserts

        // read unknown field
        let unknown = buffer.read_u32::<LittleEndian>()?;
        log::debug!("Read unknown field: {}", unknown);
        assert_eq!(unknown, 1);

        // read 'hsin' tag
        let mut hsin_tag: [u8; 4] = [0; 4];
        buffer.read_exact(&mut hsin_tag)?;
        assert_eq!(hsin_tag, [104, 115, 105, 110]); // hsin

        // read hsin id
        let hsin_id = buffer.read_u64::<LittleEndian>()?;
        log::debug!("Read hsin id: {}", hsin_id);
        assert_eq!(hsin_id, 1);

        // read uuid
        let mut uuid: [u8; 16] = [0; 16];
        buffer.read_exact(&mut uuid)?;

        // read data chunk

        // read unk (index?) field
        // read children field
        // read unk field
        // read internal tag
        // read internal id

        // read children

        // TODO: return frame
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading_files() -> Result<(), Box<dyn std::error::Error>> {
        let paths: Vec<std::path::PathBuf> = glob::glob("data/files/**/*.*")?
            .filter_map(|path| path.ok())
            .filter(|path| path.file_name().unwrap() != ".DS_Store")
            .collect();

        // valid container files
        for path in paths {
            println!("reading {:?}", path);
            let file = std::fs::read(path)?;
            Frame::read(&file)?;
        }
        Ok(())
    }
}
