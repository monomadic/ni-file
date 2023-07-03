use crate::{read_bytes::ReadBytesExt, NIFileError};

use self::{patch_header::BPatchHeaderV42, patch_meta_info_header::BPatchMetaInfoHeader};

pub mod bparam_array;
mod filename_list;
mod patch_header;
mod patch_meta_info_header;
pub mod program_data;
mod start_criteria;
mod structured_object;
mod voice_groups;
mod voice_limit;
mod zone_list;

pub struct Kontakt2 {
    header: BPatchHeaderV42,
    program_data: Vec<u8>,
    meta_info: BPatchMetaInfoHeader,
}

impl Kontakt2 {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, NIFileError> {
        let header = BPatchHeaderV42::read(&mut reader)?;

        let program_data = crate::decompress::decompress(
            reader.read_bytes(header.zlib_length)?.as_slice(),
            header.decompressed_length,
        )
        .expect("decompression failure");

        // TODO: loop
        let chunk = structured_object::StructuredObject::read(program_data.as_slice())?;

        let meta_info = BPatchMetaInfoHeader::read(&mut reader)?;

        Ok(Self {
            header,
            program_data,
            meta_info,
        })
    }
}

pub fn read_internal<R: ReadBytesExt>(mut reader: R) -> Result<Vec<u8>, NIFileError> {
    let decompressed_length = reader.read_u32_le()? as usize;
    let _unknown = reader.read_bytes(41)?;
    reader
        .read_bytes(decompressed_length)
        .map_err(NIFileError::from)
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
            Kontakt2::read(file)?;
        }

        Ok(())
    }
}
