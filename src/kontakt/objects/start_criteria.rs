use std::io::Cursor;

use crate::{
    kontakt::{Chunk, KontaktError, StructuredObject},
    read_bytes::ReadBytesExt,
    Error,
};

const CHUNK_ID: u16 = 0x0F;

/// Type:           Chunk
/// SerType:        0x0F
/// Version:        0x70
/// Kontakt 7:      BParStartCriteria
/// KontaktIO:      K4PL_StartCriteria
#[derive(Debug)]
pub struct StartCriteria(pub StructuredObject);

#[derive(Debug)]
pub struct StartCriteriaParams {
    /// Mode: Always, Start On Key, Start On Controller, Cycle Round Robin, Cycle Random, Slice Trigger
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

impl StartCriteriaParams {
    // 33 bytes
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        Ok(StartCriteriaParams {
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

impl StartCriteria {
    pub fn params(&self) -> Result<StartCriteriaParams, Error> {
        StartCriteriaParams::read(&mut Cursor::new(&self.0.public_data))
    }
}

impl std::convert::TryFrom<&Chunk> for StartCriteria {
    type Error = Error;

    fn try_from(chunk: &Chunk) -> Result<Self, Self::Error> {
        if chunk.id != CHUNK_ID {
            return Err(KontaktError::IncorrectID {
                expected: CHUNK_ID,
                got: chunk.id,
            }
            .into());
        }
        Ok(Self(chunk.try_into()?))
    }
}
