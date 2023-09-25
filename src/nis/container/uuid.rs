use std::fmt::Display;

use crate::{read_bytes::ReadBytesExt, NIFileError};

#[derive(Debug)]
pub struct Uuid {
    time_low: u32,
    time_mid: u16,
    time_high: u16,
    clock_sequence_high: u8,
    clock_sequence_low: u8,
    node_1: u8,
    node_2: u8,
    node_3: u8,
    node_4: u8,
    node_5: u8,
    node_6: u8,
}

impl Display for Uuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{:x}-{:x}-{:x}-{:x}{:x}-{:x}{:x}{:x}{:x}{:x}{:x}",
            self.time_low,
            self.time_mid,
            self.time_high,
            self.clock_sequence_high,
            self.clock_sequence_low,
            self.node_1,
            self.node_1,
            self.node_1,
            self.node_1,
            self.node_1,
            self.node_1,
        ))
    }
}

impl Uuid {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, NIFileError> {
        Ok(Self {
            time_low: reader.read_u32_le()?,
            time_mid: reader.read_u16_le()?,
            time_high: reader.read_u16_le()?,
            clock_sequence_high: reader.read_u8()?,
            clock_sequence_low: reader.read_u8()?,
            node_1: reader.read_u8()?,
            node_2: reader.read_u8()?,
            node_3: reader.read_u8()?,
            node_4: reader.read_u8()?,
            node_5: reader.read_u8()?,
            node_6: reader.read_u8()?,
        })
    }
}
