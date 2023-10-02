use crate::{read_bytes::ReadBytesExt, Error};

use super::XMLDocument;

#[derive(Debug)]
pub struct Kon1 {
    pub preset: XMLDocument,
}

impl Kon1 {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        // V1 headers are always Kon1 files.
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;

        Ok(Kon1 {
            preset: XMLDocument::from_compressed_data(data.as_slice())?,
        })
    }
}
