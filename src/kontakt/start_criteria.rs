use crate::{read_bytes::ReadBytesExt, NIFileError};

/// mode
/// nextCriteria
/// key_min
/// key_max
/// controller
/// cc_min
/// cc_max
/// cycleClass
/// sliceZoneIdx
/// sliceZoneSliceIdx
/// sequencerOnly

pub struct StartCriteria {
    // mode: i32,
    // next_criteria: i32,
    // key_min: i16,
    // key_max: i16,
    // controller: i16,
    // cc_min: i16,
    // cc_max: i16,
    // cycle_class: i32,
    // slice_zone_idx: i32,
    // slice_zone_slice_idx: i32,
    // sequencer_only: bool,
}

// id: 0x28
impl StartCriteria {
    pub fn read<R: ReadBytesExt>(_reader: R) -> Result<Self, NIFileError> {
        Ok(Self {})
    }
}
