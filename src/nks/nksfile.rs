use crate::{
    kontakt::{chunkdata::ChunkData, structured_object::StructuredObject},
    nks::meta_info::BPatchMetaInfoHeader,
    read_bytes::ReadBytesExt,
    Error, NIFileError,
};

use super::header::NKSHeader;

#[derive(Debug)]
pub struct NKSFile {
    pub header: NKSHeader,
    pub data: Vec<u8>,
    pub meta_info: BPatchMetaInfoHeader,
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
            NKSHeader::BPatchHeaderV42(h) => {
                // deflate InternalPatchData
                let data = crate::deflate::deflate_checked(
                    reader.read_bytes(zlib_length)?.as_slice(),
                    h.decompressed_length as usize,
                )?;

                Ok(NKSFile {
                    header,
                    data,
                    meta_info: BPatchMetaInfoHeader::read(&mut reader)?,
                })
            }
        }
    }

    pub fn data(&self) -> Result<Vec<StructuredObject>, Error> {
        let mut objects = Vec::new();

        while let Ok(chunk) = ChunkData::read(self.data.as_slice()) {
            let mut reader = chunk.data.as_slice();
            objects.push(StructuredObject::read(&mut reader)?);
        }

        Ok(objects)
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
