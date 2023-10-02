use std::io::Cursor;

use crate::{kontakt::structured_object::StructuredObject, read_bytes::ReadBytesExt, Error};

#[derive(Debug)]
pub struct ZoneData(StructuredObject);

#[derive(Debug)]
pub struct ZoneDataPublicParams {
    pub sample_start: i32,
    pub sample_end: i32,
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
    pub filename_id: i32,
    sample_data_type: i32,
    sample_rate: i32,
    num_channels: u8,
    num_frames: i32,
    reserved1: i32,
    reserved2: Option<i32>,
    pub root_note: i32,
    tuning: f32,
    reserved3: bool,
    reserved4: i32,
}

impl ZoneData {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        Ok(Self(StructuredObject::read(&mut reader)?))
    }

    pub fn public_params(&self) -> Result<ZoneDataPublicParams, Error> {
        let mut reader = Cursor::new(&self.0.public_data);

        Ok(ZoneDataPublicParams {
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
            reserved2: {
                match self.0.version {
                    _ if self.0.version < 0x96 => Some(reader.read_i32_le()?),
                    _ if self.0.version < 0x99 => None,
                    _ => None,
                }
            },
            root_note: reader.read_i32_le()?,
            tuning: reader.read_f32_le()?,
            reserved3: reader.read_bool()?,
            reserved4: reader.read_i32_le()?,
        })
    }
}

// #[derive(Debug)]
// pub struct ZoneDataV98 {
//     pub sample_start: i32,
//     pub sample_end: i32,
//     sample_start_mod_range: i32,
//     low_velocity: i16,
//     high_velocity: i16,
//     low_key: i16,
//     high_key: i16,
//     fade_low_velocity: i16,
//     fade_high_velocity: i16,
//     fade_low_key: i16,
//     fade_high_key: i16,
//     root_key: i16,
//     zone_volume: f32,
//     zone_pan: f32,
//     zone_tune: f32,
//     pub filename_id: i32,
//     sample_data_type: i32,
//     sample_rate: i32,
//     num_channels: u8,
//     num_frames: i32,
//     reserved1: i32,
//     pub root_note: i32,
//     tuning: f32,
//     reserved3: bool,
//     reserved4: i32,
// }
//
// impl ZoneDataV98 {
//     pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
//         Ok(ZoneDataV98 {
//             sample_start: reader.read_i32_le()?,
//             sample_end: reader.read_i32_le()?,
//             sample_start_mod_range: reader.read_i32_le()?,
//             low_velocity: reader.read_i16_le()?,
//             high_velocity: reader.read_i16_le()?,
//             low_key: reader.read_i16_le()?,
//             high_key: reader.read_i16_le()?,
//             fade_low_velocity: reader.read_i16_le()?,
//             fade_high_velocity: reader.read_i16_le()?,
//             fade_low_key: reader.read_i16_le()?,
//             fade_high_key: reader.read_i16_le()?,
//             root_key: reader.read_i16_le()?,
//             zone_volume: reader.read_f32_le()?,
//             zone_pan: reader.read_f32_le()?,
//             zone_tune: reader.read_f32_le()?,
//             filename_id: reader.read_i32_le()?,
//             sample_data_type: reader.read_i32_le()?,
//             sample_rate: reader.read_i32_le()?,
//             num_channels: reader.read_u8()?,
//             num_frames: reader.read_i32_le()?,
//             reserved1: reader.read_i32_le()?,
//             root_note: reader.read_i32_le()?,
//             tuning: reader.read_f32_le()?,
//             reserved3: reader.read_bool()?,
//             reserved4: reader.read_i32_le()?,
//         })
//     }
// }
//
// #[derive(Debug)]
// pub struct ZoneDataV95 {
//     pub sample_start: i32,
//     pub sample_end: i32,
//     sample_start_mod_range: i32,
//     low_velocity: i16,
//     high_velocity: i16,
//     low_key: i16,
//     high_key: i16,
//     fade_low_velocity: i16,
//     fade_high_velocity: i16,
//     fade_low_key: i16,
//     fade_high_key: i16,
//     root_key: i16,
//     zone_volume: f32,
//     zone_pan: f32,
//     zone_tune: f32,
//     pub filename_id: i32,
//     sample_data_type: i32,
//     sample_rate: i32,
//     num_channels: u8,
//     num_frames: i32,
//     reserved1: i32,
//     reserved2: i32,
//     pub root_note: i32,
//     tuning: f32,
//     reserved3: bool,
//     reserved4: i32,
// }
//
// impl ZoneDataV95 {
//     pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
//         Ok(ZoneDataV95 {
//             sample_start: reader.read_i32_le()?,
//             sample_end: reader.read_i32_le()?,
//             sample_start_mod_range: reader.read_i32_le()?,
//             low_velocity: reader.read_i16_le()?,
//             high_velocity: reader.read_i16_le()?,
//             low_key: reader.read_i16_le()?,
//             high_key: reader.read_i16_le()?,
//             fade_low_velocity: reader.read_i16_le()?,
//             fade_high_velocity: reader.read_i16_le()?,
//             fade_low_key: reader.read_i16_le()?,
//             fade_high_key: reader.read_i16_le()?,
//             root_key: reader.read_i16_le()?,
//             zone_volume: reader.read_f32_le()?,
//             zone_pan: reader.read_f32_le()?,
//             zone_tune: reader.read_f32_le()?,
//             filename_id: reader.read_i32_le()?,
//             sample_data_type: reader.read_i32_le()?,
//             sample_rate: reader.read_i32_le()?,
//             num_channels: reader.read_u8()?,
//             num_frames: reader.read_i32_le()?,
//             reserved1: reader.read_i32_le()?,
//             reserved2: reader.read_i32_le()?,
//             root_note: reader.read_i32_le()?,
//             tuning: reader.read_f32_le()?,
//             reserved3: reader.read_bool()?,
//             reserved4: reader.read_i32_le()?,
//         })
//     }
// }
