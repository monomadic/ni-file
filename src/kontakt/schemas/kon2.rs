use crate::{read_bytes::ReadBytesExt, Error};

use super::XMLDocument;

// #[derive(Debug)]
// pub struct Kon2 {
//     pub zlib_length: u32,
//     pub decompressed_length: u32,
//     pub compressed_data: Vec<u8>,
//     pub meta_info: BPatchMetaInfoHeader,
// }

#[derive(Debug)]
pub struct Kon2 {
    pub preset: XMLDocument,
}

impl Kon2 {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        // V1 headers are always Kon1 files.
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;

        Ok(Kon2 {
            preset: XMLDocument::from_compressed_data(data.as_slice())?,
        })
    }
}
