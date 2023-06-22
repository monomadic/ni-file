pub struct ProgramDataVA5 {
    name: String,

    num_bytes_samples_total: f32,
    transpose: u8,
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

// impl ProgramDataVA5 {
//     pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {}
// }
