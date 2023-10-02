use flate2::bufread::ZlibDecoder;
use std::fmt::Display;
use std::io::Read;

use crate::nks::error::NKSError;

#[derive(Debug)]
pub struct XMLDocument(String);

impl XMLDocument {
    pub fn from_compressed_data(data: &[u8]) -> Result<Self, NKSError> {
        let mut decoder = ZlibDecoder::new(data);
        let mut decompressed_data = Vec::new();
        decoder.read_to_end(&mut decompressed_data)?;

        Ok(XMLDocument(
            String::from_utf8(decompressed_data).expect("convert xml"),
        ))
    }
}

impl Display for XMLDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}
