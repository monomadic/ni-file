use crate::{read_bytes::ReadBytesExt, Error};

/// Kontakt7: StartCriteria
#[derive(Debug)]
pub struct StartCritList;

impl StartCritList {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        dbg!(reader.read_u8()?);
        Ok(Self {})
    }
}
