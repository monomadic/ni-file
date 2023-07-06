use crate::{read_bytes::ReadBytesExt, Error};

pub struct GroupList;

impl GroupList {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        Ok(Self {})
    }
}
