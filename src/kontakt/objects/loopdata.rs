use crate::{read_bytes::ReadBytesExt, Error};

pub struct Loop {
    mode: i32,
    loop_start: i32,
    loop_length: i32,
    loop_count: i32,
    alternating_loop: bool,
    loop_tuning: f32,
    x_fade_length: i32,
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
