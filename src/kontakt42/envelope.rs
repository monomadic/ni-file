pub struct Envelope {}

use crate::{read_bytes::ReadBytesExt, Error};

#[derive(Debug)]
pub struct EnvelopeAHDSR_V10;

impl EnvelopeAHDSR_V10 {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        Ok(Self {})
    }
}
