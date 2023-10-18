use crate::{read_bytes::ReadBytesExt, Error};

// TODO: change to Chunk reader

/// Type:           Chunk
/// SerType:        0x05
/// Kontakt 7:      BLoop
/// KontaktIO:      K4PL_Loop
#[derive(Debug)]
pub struct Loop {
    pub mode: i32,
    pub loop_start: i32,
    pub loop_length: i32,
    pub loop_count: i32,
    pub alternating_loop: bool,
    pub loop_tuning: f32,
    pub x_fade_length: i32,
}

impl Loop {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        Ok(Self {
            mode: reader.read_i32_le()?,
            loop_start: reader.read_i32_le()?,
            loop_length: reader.read_i32_le()?,
            loop_count: reader.read_i32_le()?,
            alternating_loop: reader.read_bool()?,
            loop_tuning: reader.read_f32_le()?,
            x_fade_length: reader.read_i32_le()?,
        })
    }
}
