use crate::{read_bytes::ReadBytesExt, Error};

/// Type:           Chunk
/// SerType:        0x0F
/// Version:        0x70
/// Kontakt 7:      BParStartCriteria
/// KontaktIO:      K4PL_StartCriteria
pub struct StartCriteria {
    mode: i32,
    next_criteria: i32,
    key_min: i16,
    key_max: i16,
    controller: i16,
    cc_min: i16,
    cc_max: i16,
    cycle_class: i32,
    slice_zone_idx: i32,
    slice_zone_slice_idx: i32,
    sequencer_only: bool,
}

impl StartCriteria {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        Ok(Self {
            mode: reader.read_i32_le()?,
            next_criteria: reader.read_i32_le()?,
            key_min: reader.read_i16_le()?,
            key_max: reader.read_i16_le()?,
            controller: reader.read_i16_le()?,
            cc_min: reader.read_i16_le()?,
            cc_max: reader.read_i16_le()?,
            cycle_class: reader.read_i32_le()?,
            slice_zone_idx: reader.read_i32_le()?,
            slice_zone_slice_idx: reader.read_i32_le()?,
            sequencer_only: reader.read_bool()?,
        })
    }
}
