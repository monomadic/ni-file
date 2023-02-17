use crate::read_bytes::ReadBytesExt;
use thiserror::Error;

use super::header::FrameHeader;

/// named type for a raw frame
pub struct Frame(pub Vec<u8>);
pub struct DataFrame(pub Vec<u8>);

#[derive(Error, Debug)]
pub enum FrameError {
    #[error("Size field mismatch: expected {expected}, got {got}")]
    IncorrectFrameSize { expected: u64, got: u64 },

    #[error("IO Error")]
    IO(#[from] std::io::Error),
}

impl Frame {
    /// read a byte stream into a raw Frame
    pub fn read<R>(reader: R) -> Result<Frame, FrameError>
    where
        R: ReadBytesExt,
    {
        Ok(Frame(read_frame_data(reader)?))
    }

    pub fn header(&self) -> Result<FrameHeader, FrameError> {
        let slice = self.0.as_slice().read_bytes(20)?;
        let frameheader = FrameHeader::read(slice.as_slice())?;
        Ok(frameheader)
    }

    pub fn data_frame(&self) -> Result<DataFrame, FrameError> {
        let data = self.0.clone();
        let mut data = data.as_slice();
        let _ = data.read_bytes(20)?; // header
        let data_frame = DataFrame(read_frame_data(data)?);
        Ok(data_frame)
    }
}

/// checks a frame is a valid size and returns its contents as a byte array
pub fn read_frame_data<R>(mut reader: R) -> Result<Vec<u8>, FrameError>
where
    R: ReadBytesExt,
{
    // read size field
    // method.NI::SOUND::ItemFrame.getFrameSize
    let size_field = reader.read_u64_le()?;
    log::debug!("size field: {}", size_field);

    // read data into buffer
    let size_field_len = std::mem::size_of::<u64>();
    Ok(reader.read_bytes(size_field as usize - size_field_len)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_test() -> Result<(), Box<dyn std::error::Error>> {
        let bytes = [12_u64.to_le_bytes().to_vec(), 64_u32.to_le_bytes().to_vec()].concat();
        assert_eq!(read_frame_data(bytes.as_slice())?, 64_u32.to_le_bytes());
        Ok(())
    }

    #[test]
    fn test_reading_files() -> Result<(), Box<dyn std::error::Error>> {
        crate::utils::setup_logger();

        for path in crate::utils::get_test_files()? {
            log::info!("reading {:?}", path);
            let file = std::fs::read(&path)?;
            let bytes = read_frame_data(file.as_slice())?;

            assert_eq!(bytes.len(), file.len() - 8);
        }
        Ok(())
    }
}
