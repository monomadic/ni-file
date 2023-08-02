use crate::{read_bytes::ReadBytesExt, Error, NIFileError};

#[derive(Debug)]
pub struct ProgramDataV80 {
    name: String,
    num_bytes_samples_total: f64,
    transpose: i8,
    volume: f32,
    pan: f32,
    tune: f32,
    low_velocity: u8,
    high_velocity: u8,
    low_key: u8,
    high_key: u8,
    default_key_switch: i16,
    dfd_channel_preload_size: i32,
    library_id: i32,
    fingerprint: u32,
    loading_flags: u32,
    group_solo: bool,
    cat_icon_idx: i32,
    instrument_credits: String,
    instrument_author: String,
    instrument_url: String,
    instrument_cat1: i16,
    instrument_cat2: i16,
    instrument_cat3: i16,
}

impl ProgramDataV80 {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, NIFileError> {
        Ok(Self {
            name: reader.read_widestring_utf16()?,
            num_bytes_samples_total: reader.read_f64_le()?,
            transpose: reader.read_i8()?,
            volume: reader.read_f32_le()?,
            pan: reader.read_f32_le()?,
            tune: reader.read_f32_le()?,
            low_velocity: reader.read_u8()?,
            high_velocity: reader.read_u8()?,
            low_key: reader.read_u8()?,
            high_key: reader.read_u8()?,
            default_key_switch: reader.read_i16_le()?,
            dfd_channel_preload_size: reader.read_i32_le()?,
            library_id: reader.read_i32_le()?,
            fingerprint: reader.read_u32_le()?,
            loading_flags: reader.read_u32_le()?,
            group_solo: reader.read_bool()?,
            cat_icon_idx: reader.read_i32_le()?,
            instrument_credits: reader.read_widestring_utf16()?,
            instrument_author: reader.read_widestring_utf16()?,
            instrument_url: reader.read_widestring_utf16()?,
            instrument_cat1: reader.read_i16_le()?,
            instrument_cat2: reader.read_i16_le()?,
            instrument_cat3: reader.read_i16_le()?,
        })
    }
}

#[derive(Debug)]
pub struct ProgramDataVA5 {
    name: String,
    num_bytes_samples_total: f64,
    transpose: i8,
    volume: f32,
    pan: f32,
    tune: f32,
    low_velocity: u8,
    high_velocity: u8,
    low_key: u8,
    high_key: u8,
    default_key_switch: i16,
    dfd_channel_preload_size: i32,
    library_id: i32,
    fingerprint: u32,
    loading_flags: u32,
    group_solo: bool,
    cat_icon_idx: i32,
    instrument_credits: String,
    instrument_author: String,
    instrument_url: String,
    instrument_cat1: i16,
    instrument_cat2: i16,
    instrument_cat3: i16,
    resource_container_filename: i32,
    wallpaper_filename: i32,
}

impl ProgramDataVA5 {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, NIFileError> {
        Ok(Self {
            name: reader.read_widestring_utf16()?,
            num_bytes_samples_total: reader.read_f64_le()?,
            transpose: reader.read_i8()?,
            volume: reader.read_f32_le()?,
            pan: reader.read_f32_le()?,
            tune: reader.read_f32_le()?,
            low_velocity: reader.read_u8()?,
            high_velocity: reader.read_u8()?,
            low_key: reader.read_u8()?,
            high_key: reader.read_u8()?,
            default_key_switch: reader.read_i16_le()?,
            dfd_channel_preload_size: reader.read_i32_le()?,
            library_id: reader.read_i32_le()?,
            fingerprint: reader.read_u32_le()?,
            loading_flags: reader.read_u32_le()?,
            group_solo: reader.read_bool()?,
            cat_icon_idx: reader.read_i32_le()?,
            instrument_credits: reader.read_widestring_utf16()?,
            instrument_author: reader.read_widestring_utf16()?,
            instrument_url: reader.read_widestring_utf16()?,
            instrument_cat1: reader.read_i16_le()?,
            instrument_cat2: reader.read_i16_le()?,
            instrument_cat3: reader.read_i16_le()?,
            resource_container_filename: reader.read_i32_le()?,
            wallpaper_filename: reader.read_i32_le()?,
        })
    }
}

#[test]
fn test_pubdata_0x28_0x80() -> Result<(), Error> {
    let mut file = include_bytes!("../tests/ProgramData/0x28-0x80").as_slice();
    ProgramDataV80::read(&mut file)?;
    Ok(())
}
