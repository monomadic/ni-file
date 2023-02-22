use crate::prelude::*;

pub struct SubtreeItem(Vec<u8>);

impl SubtreeItem {
    fn read_item(&self) -> Result<Vec<u8>, NIFileError> {
        // read Item (or skip 24 bytes)

        // u32 version = 1
        // u32
        Ok(vec![])
    }
}
