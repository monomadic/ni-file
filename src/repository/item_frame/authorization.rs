use crate::prelude::*;
use crate::read_bytes::ReadBytesExt;

pub struct SubtreeItem(Vec<u8>);

impl SubtreeItem {
    fn read_item(&self) -> Result<Vec<u8>, NIFileError> {
        Ok(vec![])
    }
}
