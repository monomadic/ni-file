use crate::{
    kontakt::internal_patch_data::InternalPatchData, nks::meta_info::BPatchMetaInfoHeader,
    read_bytes::ReadBytesExt, NIFileError,
};

use super::header::NKSHeader;

#[derive(Debug)]
pub struct NKSFile {
    header: NKSHeader,
    data: InternalPatchData,
    meta_info: BPatchMetaInfoHeader,
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

        match &header {
            NKSHeader::BPatchHeaderV2(_) => unimplemented!(),
            NKSHeader::BPatchHeaderV42(h42) => {
                // deflate InternalPatchData
                let decompressed_data = crate::deflate::deflate_checked(
                    reader.read_bytes(zlib_length)?.as_slice(),
                    h42.decompressed_length as usize,
                )?;

                Ok(NKSFile {
                    header,
                    data: InternalPatchData::read(&mut decompressed_data.as_slice())?,
                    meta_info: BPatchMetaInfoHeader::read(&mut reader)?,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nksfile_read_v42() -> Result<(), NIFileError> {
        // let file = include_bytes!("../../tests/filetypes/nks/4.2.2.4504/000.nki");
        let file = include_bytes!("../../tests/filetypes/nks/4.2.4.5316/000.nki");
        println!("{:?}", NKSFile::read(file.as_slice())?);
        Ok(())
    }

    // #[test]
    // fn test_nksfile_read_v2() -> Result<(), NIFileError> {
    //     // let file = include_bytes!("../../tests/filetypes/nks/2.0.1.002/000.nki");
    //     let file = include_bytes!("../../tests/filetypes/nks/2.1.0.001/000.nki");
    //     // let file = include_bytes!("../../tests/filetypes/nks/4.2.2.4504/000.nki");
    //     // let file = include_bytes!("../../tests/filetypes/nks/4.2.4.5316/000.nki");
    //     println!("{:?}", NKSFile::read(file.as_slice())?);
    //     Ok(())
    // }
}
