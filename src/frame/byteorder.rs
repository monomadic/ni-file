use byteorder::{LittleEndian, ReadBytesExt};
use std::io::prelude::*;
use thiserror::Error;

fn read_bytes<R>(reader: R, bytes_to_read: u64) -> Vec<u8>
where
    R: Read,
{
    let mut buf = vec![];
    let mut chunk = reader.take(bytes_to_read);
    // Do appropriate error handling for your situation
    // Maybe it's OK if you didn't read enough bytes?
    let n = chunk.read_to_end(&mut buf).expect("Didn't read enough");
    assert_eq!(bytes_to_read as usize, n);
    buf
}

pub struct RawFrame<'a> {
    pub data: &'a [u8],
}

impl<'a> RawFrame<'a> {
    pub fn new(bytes: &'a [u8]) -> Result<RawFrame<'a>, FrameError> {
        Ok(Self { data: read(bytes)? })
    }

    pub fn validate(&self) -> Result<(), FrameError> {
        todo!()
    }

    pub fn children(&self) -> Result<Vec<RawFrame>, FrameError> {
        read_bytes(self.data, 20);
        Ok(vec![])
    }
}

#[derive(Error, Debug)]
pub enum FrameError {
    #[error("Incorrect Size Field: expected {expected}, got {got}")]
    IncorrectFrameSize { expected: u64, got: u64 },

    #[error("IO Error")]
    IO(#[from] std::io::Error),
}

pub fn read<'a>(mut bytes: &'a [u8]) -> Result<&'a [u8], FrameError> {
    // read size field
    let buffer_size = bytes.len() as u64;
    log::debug!("Received buffer size: {}", buffer_size);

    let size_field = bytes.read_u64::<LittleEndian>()?;
    log::debug!("Read size field: {}", size_field);

    if buffer_size != size_field {
        return Err(FrameError::IncorrectFrameSize {
            expected: size_field,
            got: buffer_size,
        });
    }

    let bytes_to_read: usize = (buffer_size - size_field) as usize;
    let mut buf = vec![0u8; bytes_to_read];
    bytes.read_exact(&mut buf)?;

    Ok(&bytes)
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
            let bytes = read(&file)?;

            assert_eq!(bytes.len(), file.len() - 8);
        }
        Ok(())
    }
}

// use std::convert::TryInto;
// use std::io::Read;
// fn read_bytes<T, R>(reader: &mut R) -> Result<T, std::io::Error>
// where
//     T: TryInto<Vec<u8>, Error = std::array::TryFromSliceError>,
// {
//     let size = std::mem::size_of::<T>();
//     let mut buffer = vec![0; size];
//     reader.read_exact(&mut buffer)?;
//     let arr = buffer.as_slice().try_into()?;
//     Ok(T::from_le_bytes(arr))
// }
