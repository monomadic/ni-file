// Groups allow you to apply settings like effects, volume, panning, etc. to multiple samples at once rather than having to adjust each one individually.

use std::io::Cursor;

use crate::{
    kontakt::{objects::start_criteria_list::StartCriteriaList, StructuredObject},
    read_bytes::ReadBytesExt,
    Error,
};

/// Type:           Chunk
/// SerType:        0x04
/// Kontakt 7:      BGroup?
/// KontaktIO:      K4PL\_Group
#[derive(Debug)]
pub struct Group(pub StructuredObject);

#[derive(Debug)]
pub struct GroupParams {
    pub name: String,
    pub volume: f32,
    pub pan: f32,
    /// Change pitch in semitones.
    pub tune: f32,
    /// Repitch samples to midi note triggered.
    pub key_tracking: bool,
    /// Play samples in reverse.
    pub reverse: bool,
    pub release_trigger: bool,
    pub release_trigger_note_monophonic: bool,
    pub rls_trig_counter: i32,
    pub midi_channel: i16,
    pub voice_group_index: i32,
    pub fx_idx_amp_split_point: i32,
    pub muted: bool,
    pub soloed: bool,
    pub interp_quality: i32,
    // v90
    //     BParameterArraySerBParInternalMod16 0x3B
    //     BParameterArraySerBParExternalMod32 0x3C
    pub start_criteria: StartCriteriaList,
    // v95
    //     BParGroupDynamics 0x4A
}

impl Group {
    pub fn params(&self) -> Result<GroupParams, Error> {
        let mut reader = Cursor::new(&self.0.public_data);

        Ok(GroupParams {
            name: reader.read_widestring_utf16()?,
            volume: reader.read_f32_le()?,
            pan: reader.read_f32_le()?,
            tune: reader.read_f32_le()?,
            key_tracking: reader.read_bool()?,
            reverse: reader.read_bool()?,
            release_trigger: reader.read_bool()?,
            release_trigger_note_monophonic: reader.read_bool()?,
            rls_trig_counter: reader.read_i32_le()?,
            midi_channel: reader.read_i16_le()?,
            voice_group_index: reader.read_i32_le()?,
            fx_idx_amp_split_point: reader.read_i32_le()?,
            muted: reader.read_bool()?,
            soloed: reader.read_bool()?,
            interp_quality: reader.read_i32_le()?,
            start_criteria: (&self.0.children[2]).try_into()?,
        })
    }
}
