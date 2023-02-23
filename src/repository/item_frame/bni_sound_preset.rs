/*
    BNISoundPreset (0x3, 3, 4KIN)
    kontakt preset

    BNISoundPreset::readItem(&stream, context) {
        let header = ItemFrameReader(&context);
        let preset = Preset::readItem(&stream, &context)?;
        let version = context.read_u16();
        if version != 0 {
            return Err(VERSION_MISMATCH)
        }
    }
*/

use crate::prelude::*;
use crate::read_bytes::ReadBytesExt;

#[derive(Debug)]
pub struct BNISoundPreset(Vec<u8>);

impl BNISoundPreset {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {
        Ok(Self(vec![]))
    }
}
