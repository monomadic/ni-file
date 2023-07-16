use crate::{read_bytes::ReadBytesExt, NIFileError};

use super::header::NKSHeader;

#[derive(Debug)]
pub struct NKSFile {
    header: NKSHeader,
}

impl NKSFile {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, NIFileError> {
        let magic = reader.read_u32_le()?;
        assert_eq!(
            magic, 0x7fa89012,
            "Stream does not appear to be NKS Little Endian"
        );

        let zlib_length = reader.read_u32_le()? as usize;
        let header = NKSHeader::read_le(&mut reader)?;

        Ok(NKSFile { header })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nksfile_read() -> Result<(), NIFileError> {
        // let file = include_bytes!("../../tests/filetypes/nks/2.0.1.002/000.nki");
        let file = include_bytes!("../../tests/filetypes/nks/2.1.0.001/000.nki");
        // let file = include_bytes!("../../tests/filetypes/nks/4.2.2.4504/000.nki");
        // let file = include_bytes!("../../tests/filetypes/nks/4.2.4.5316/000.nki");
        println!("{:?}", NKSFile::read(file.as_slice())?);
        Ok(())
    }
}
