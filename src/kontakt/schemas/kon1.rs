use crate::{read_bytes::ReadBytesExt, Error};

use super::XMLDocument;

#[derive(Debug)]
pub struct KontaktV1 {
    pub preset: XMLDocument,
}

impl KontaktV1 {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        // V1 headers are always Kon1 files.
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;

        Ok(KontaktV1 {
            preset: XMLDocument::from_utf8(&data).expect("xml error"),
        })
    }
}
