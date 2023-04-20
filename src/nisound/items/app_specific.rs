use crate::prelude::*;

pub struct AppSpecific;

impl AppSpecific {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {}
}
