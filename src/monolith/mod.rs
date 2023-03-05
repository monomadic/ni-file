use crate::prelude::*;
use crate::read_bytes::ReadBytesExt;

pub struct NIMonolith(Vec<u8>);

impl NIMonolith {
    pub fn read<R: ReadBytesExt>(_reader: R) -> Result<Self> {
        log::debug!("NIMonolith::read");
        Ok(Self(Vec::new()))
    }
}
