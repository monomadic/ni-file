use crate::read_bytes::ReadBytesExt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FrameError {
    #[error("Size field mismatch: expected {expected}, got {got}")]
    IncorrectFrameSize { expected: u64, got: u64 },

    #[error("IO Error")]
    IO(#[from] std::io::Error),
}

/// checks a frame is a valid size and returns its contents as a byte array
pub fn read_frame_data<R>(mut reader: R) -> Result<Vec<u8>, FrameError>
where
    R: ReadBytesExt,
{
    // read size field
    let size_field = reader.read_u64_le()? as usize;
    log::debug!("Size field: {}", size_field);

    reader
        .read_bytes(size_field - 4)
        .map_err(|_| FrameError::IncorrectFrameSize {
            expected: size_field as u64,
            got: 0, // FIXME: don't use 0, make new error type
        })
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
        let paths: Vec<std::path::PathBuf> = glob::glob("data/files/**/*.*")?
            .filter_map(|path| path.ok())
            .filter(|path| path.file_name().unwrap() != ".DS_Store")
            .collect();

        // valid container files
        for path in paths {
            println!("reading {:?}", path);

            let file = std::fs::read(path)?;
            let bytes = read_frame_data(file.as_slice())?;

            assert_eq!(bytes.len(), file.len() - 8);
        }
        Ok(())
    }
}
