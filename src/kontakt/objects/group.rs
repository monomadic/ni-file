// Groups allow you to apply settings like effects, volume, panning, etc. to multiple samples at once rather than having to adjust each one individually.

use std::io::Cursor;

use crate::{
    kontakt::{objects::start_crit_list::StartCritList, StructuredObject},
    read_bytes::ReadBytesExt,
    Error,
};

#[derive(Debug)]
pub struct Group(pub StructuredObject);

#[derive(Debug)]
pub struct GroupParams {
    name: String,
    volume: f32,
    pan: f32,
    tune: f32,
    key_tracking: bool,
    reverse: bool,
    release_trigger: bool,
    release_trigger_note_monophonic: bool,
    rls_trig_counter: i32,
    midi_channel: i16,
    voice_group_index: i32,
    fx_idx_amp_split_point: i32,
    muted: bool,
    soloed: bool,
    interp_quality: i32,
    // v90
    //     BParameterArraySerBParInternalMod16 0x3B
    //     BParameterArraySerBParExternalMod32 0x3C
    //     StartCritList 0x38
    // v95
    //     BParGroupDynamics 0x4A
}

impl Group {
    pub fn params(&self) -> Result<GroupParams, Error> {
        let mut reader = Cursor::new(&self.0.public_data);

        println!("{} {}", self.0.children.len(), self.0.version);

        for chunk in &self.0.children {
            println!("{:?} 0x{:x}", chunk.into_object()?, chunk.id);
        }

        // let start_crit_list: StartCritList = self.0.children[1].try_into()?;

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
        })
    }
}
