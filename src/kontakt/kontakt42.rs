use crate::{read_bytes::ReadBytesExt, NIFileError};

use super::{
    patch_header::BPatchHeaderV42, patch_meta_info_header::BPatchMetaInfoHeader,
    structured_object::StructuredObject,
};

pub struct KontaktV42 {
    header: BPatchHeaderV42,
    objects: Vec<StructuredObject>,
    meta_info: BPatchMetaInfoHeader,
}

impl KontaktV42 {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, NIFileError> {
        let header = BPatchHeaderV42::read(&mut reader)?;
        println!("{:?}", header);

        let decompressed_data = crate::decompress::decompress(
            reader.read_bytes(header.zlib_length)?.as_slice(),
            header.decompressed_length,
        )?;

        let mut decompressed_data = decompressed_data.as_slice();

        let mut objects = Vec::new();
        objects.push(StructuredObject::read(&mut decompressed_data)?);
        objects.push(StructuredObject::read(&mut decompressed_data)?);

        Ok(Self {
            header,
            objects,
            meta_info: BPatchMetaInfoHeader::read(&mut reader)?,
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
        for path in crate::utils::get_files("tests/data/kontakt-42/**/*.nki")? {
            println!("reading {:?}", path);

            let file = std::fs::File::open(&path)?;
            KontaktV42::read(file)?;
        }

        Ok(())
    }
}
