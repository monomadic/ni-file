use crate::{read_bytes::ReadBytesExt, Error};

use super::XMLDocument;

/// Basically adds monolith (resource) support to Kontakt1
#[derive(Debug)]
pub struct KontaktV2 {
    pub preset: XMLDocument,
}

impl KontaktV2 {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        // V1 headers are always Kon1 files.
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;

        Ok(KontaktV2 {
            preset: XMLDocument::from_utf8(&data).expect("xml error"),
        })
    }
}
