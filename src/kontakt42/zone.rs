use crate::{read_bytes::ReadBytesExt, Error};

#[derive(Debug)]
pub struct ZoneV98 {
    sample_start: i32,
    sample_end: i32,
    sample_start_mod_range: i32,
    low_velocity: i16,
    high_velocity: i16,
    low_key: i16,
    high_key: i16,
    fade_low_velocity: i16,
    fade_high_velocity: i16,
    fade_low_key: i16,
    fade_high_key: i16,
    root_key: i16,
    zone_volume: f32,
    zone_pan: f32,
    zone_tune: f32,
    filename_id: i32,
    sample_data_type: i32,
    sample_rate: i32,
    num_channels: u8,
    num_frames: i32,
    reserved1: i32,
    root_note: i32,
    tuning: f32,
    reserved3: bool,
    reserved4: i32,
}

impl ZoneV98 {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        println!("\nK4PL_Zone<K4PL::ZoneDataV98>::read()");
        Ok(ZoneV98 {
            sample_start: reader.read_i32_le()?,
            sample_end: reader.read_i32_le()?,
            sample_start_mod_range: reader.read_i32_le()?,
            low_velocity: reader.read_i16_le()?,
            high_velocity: reader.read_i16_le()?,
            low_key: reader.read_i16_le()?,
            high_key: reader.read_i16_le()?,
            fade_low_velocity: reader.read_i16_le()?,
            fade_high_velocity: reader.read_i16_le()?,
            fade_low_key: reader.read_i16_le()?,
            fade_high_key: reader.read_i16_le()?,
            root_key: reader.read_i16_le()?,
            zone_volume: reader.read_f32_le()?,
            zone_pan: reader.read_f32_le()?,
            zone_tune: reader.read_f32_le()?,
            filename_id: reader.read_i32_le()?,
            sample_data_type: reader.read_i32_le()?,
            sample_rate: reader.read_i32_le()?,
            num_channels: reader.read_u8()?,
            num_frames: reader.read_i32_le()?,
            reserved1: reader.read_i32_le()?,
            root_note: reader.read_i32_le()?,
            tuning: reader.read_f32_le()?,
            reserved3: reader.read_bool()?,
            reserved4: reader.read_i32_le()?,
        })
    }
}

#[derive(Debug)]
pub struct ZoneV95;

impl ZoneV95 {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        println!("\nK4PL_Zone<K4PL::ZoneDataV95>::read()");

        Ok(Self)
    }
}
