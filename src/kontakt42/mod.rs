use crate::{NIFileError, read_bytes::ReadBytesExt};

use self::patch_header::BPatchHeaderV42;

mod program_data;
mod patch_header;

pub struct Kontakt42 {
    header: BPatchHeaderV42,
    program_data: Vec<u8>
}

impl Kontakt42 {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, NIFileError> {
        let header = BPatchHeaderV42::read(&mut reader)?;

        let program_data = crate::decompress::decompress(
                reader.read_bytes(header.zlib_length)?.as_slice(),
                header.decompressed_length
            ).expect("decompression failure");

        // si header AEE10EB0 01010C00
        let _unknown = reader.read_bytes(8)?;

        let soundinfo_length = reader.read_u32_le()? as usize;
        let soundinfo = reader.read_bytes(soundinfo_length)?;
        let soundinfo = String::from_utf8(soundinfo).unwrap();
        println!("soundinfo: {}", soundinfo);

        Ok(Self {
            header,
            program_data,
        })
    }
}

pub fn read_internal<R: ReadBytesExt>(mut reader: R) -> Result<Vec<u8>, NIFileError> {
    let decompressed_length = reader.read_u32_le()? as usize;
    let _unknown = reader.read_bytes(41)?;
    reader.read_bytes(decompressed_length).map_err(NIFileError::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kontakt42_preset_read() -> Result<(), NIFileError> {
        //crate::utils::setup_logger();

        for path in crate::utils::get_files("tests/data/kontakt-42/**/*.nki")? {
            println!("reading {:?}", path);

            let file = std::fs::File::open(&path)?;
            Kontakt42::read(file)?;
        }

        Ok(())
    }
}
