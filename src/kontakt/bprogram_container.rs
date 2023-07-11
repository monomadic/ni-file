use crate::{read_bytes::ReadBytesExt, Error};

#[derive(Debug)]
pub struct BProgramContainer {}

impl BProgramContainer {
    pub fn read<R: ReadBytesExt>(mut reader: R, version: u16) -> Result<(), Error> {
        Ok(())
    }
}
