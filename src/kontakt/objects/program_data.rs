use crate::{read_bytes::ReadBytesExt, Error, NIFileError};

#[derive(Debug, Default)]
pub struct ProgramPublicParams {
    pub name: String,
    pub num_bytes_samples_total: f64,
    pub transpose: i8,
    pub volume: f32,
    pub pan: f32,
    pub tune: f32,
    pub low_velocity: u8,
    pub high_velocity: u8,
    pub low_key: u8,
    pub high_key: u8,
    pub default_key_switch: i16,
    pub dfd_channel_preload_size: i32,
    pub library_id: i32,
    pub fingerprint: u32,
    pub loading_flags: u32,
    pub group_solo: bool,
    pub cat_icon_idx: i32,
    pub instrument_credits: String,
    pub instrument_author: String,
    pub instrument_url: String,
    pub instrument_cat1: i16,
    pub instrument_cat2: i16,
    pub instrument_cat3: i16,
    pub resource_container_filename: Option<i32>,
    pub wallpaper_filename: Option<i32>,
}

impl ProgramPublicParams {
    pub fn read<R: ReadBytesExt>(mut reader: R, version: u16) -> Result<Self, NIFileError> {
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
            resource_container_filename: {
                match version {
                    _ => None,
                }
            },
            wallpaper_filename: {
                match version {
                    _ => None,
                }
            },
        })
    }
}

#[derive(Debug)]
pub struct PrivParsV80;

impl PrivParsV80 {
    // BProgram::doReadPrivPars
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        // 0x80
        println!("version {:?}", reader.read_i32_le()?); // < 2
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_i32_le()?);
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_i32_le()?);
        println!("{:?}", reader.read_i32_le()?);
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_i32_le()?);
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_i32_le()?);
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_i32_le()?);
        println!("{:?}", reader.read_i32_le()?);
        println!("{:?}", reader.read_i32_le()?);
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_i16_le()?);
        println!("{:?}", reader.read_bool()?);

        println!("{:?}", reader.read_i32_le()?);
        println!("{:?}", reader.read_i32_le()?);
        Ok(Self {})
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use crate::Error;

    use super::*;

    #[test]
    fn test_private_parameters() -> Result<(), Error> {
        // Version 0x80
        let file = File::open("tests/patchdata/KontaktV42/priv_params/4.2.2.4504/000")?;
        PrivParsV80::read(file)?;

        // let file = include_bytes!("tests/structured_object/5.3.0.6464/000");
        // let mut file = file.as_slice();
        // StructuredObject::read(&mut file)?;
        // StructuredObject::read(&mut file)?;

        Ok(())
    }
}
// #[derive(Debug)]
// pub struct ProgramDataV80 {
//     name: String,
//     num_bytes_samples_total: f64,
//     transpose: i8,
//     volume: f32,
//     pan: f32,
//     tune: f32,
//     low_velocity: u8,
//     high_velocity: u8,
//     low_key: u8,
//     high_key: u8,
//     default_key_switch: i16,
//     dfd_channel_preload_size: i32,
//     library_id: i32,
//     fingerprint: u32,
//     loading_flags: u32,
//     group_solo: bool,
//     cat_icon_idx: i32,
//     instrument_credits: String,
//     instrument_author: String,
//     instrument_url: String,
//     instrument_cat1: i16,
//     instrument_cat2: i16,
//     instrument_cat3: i16,
// }
//
// impl ProgramDataV80 {
//     pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, NIFileError> {
//         Ok(Self {
//             name: reader.read_widestring_utf16()?,
//             num_bytes_samples_total: reader.read_f64_le()?,
//             transpose: reader.read_i8()?,
//             volume: reader.read_f32_le()?,
//             pan: reader.read_f32_le()?,
//             tune: reader.read_f32_le()?,
//             low_velocity: reader.read_u8()?,
//             high_velocity: reader.read_u8()?,
//             low_key: reader.read_u8()?,
//             high_key: reader.read_u8()?,
//             default_key_switch: reader.read_i16_le()?,
//             dfd_channel_preload_size: reader.read_i32_le()?,
//             library_id: reader.read_i32_le()?,
//             fingerprint: reader.read_u32_le()?,
//             loading_flags: reader.read_u32_le()?,
//             group_solo: reader.read_bool()?,
//             cat_icon_idx: reader.read_i32_le()?,
//             instrument_credits: reader.read_widestring_utf16()?,
//             instrument_author: reader.read_widestring_utf16()?,
//             instrument_url: reader.read_widestring_utf16()?,
//             instrument_cat1: reader.read_i16_le()?,
//             instrument_cat2: reader.read_i16_le()?,
//             instrument_cat3: reader.read_i16_le()?,
//         })
//     }
// }
//
// #[derive(Debug)]
// pub struct ProgramDataVA5 {
//     name: String,
//     num_bytes_samples_total: f64,
//     transpose: i8,
//     volume: f32,
//     pan: f32,
//     tune: f32,
//     low_velocity: u8,
//     high_velocity: u8,
//     low_key: u8,
//     high_key: u8,
//     default_key_switch: i16,
//     dfd_channel_preload_size: i32,
//     library_id: i32,
//     fingerprint: u32,
//     loading_flags: u32,
//     group_solo: bool,
//     cat_icon_idx: i32,
//     instrument_credits: String,
//     instrument_author: String,
//     instrument_url: String,
//     instrument_cat1: i16,
//     instrument_cat2: i16,
//     instrument_cat3: i16,
//     resource_container_filename: i32,
//     wallpaper_filename: i32,
// }
//
// impl ProgramDataVA5 {
//     pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, NIFileError> {
//         Ok(Self {
//             name: reader.read_widestring_utf16()?,
//             num_bytes_samples_total: reader.read_f64_le()?,
//             transpose: reader.read_i8()?,
//             volume: reader.read_f32_le()?,
//             pan: reader.read_f32_le()?,
//             tune: reader.read_f32_le()?,
//             low_velocity: reader.read_u8()?,
//             high_velocity: reader.read_u8()?,
//             low_key: reader.read_u8()?,
//             high_key: reader.read_u8()?,
//             default_key_switch: reader.read_i16_le()?,
//             dfd_channel_preload_size: reader.read_i32_le()?,
//             library_id: reader.read_i32_le()?,
//             fingerprint: reader.read_u32_le()?,
//             loading_flags: reader.read_u32_le()?,
//             group_solo: reader.read_bool()?,
//             cat_icon_idx: reader.read_i32_le()?,
//             instrument_credits: reader.read_widestring_utf16()?,
//             instrument_author: reader.read_widestring_utf16()?,
//             instrument_url: reader.read_widestring_utf16()?,
//             instrument_cat1: reader.read_i16_le()?,
//             instrument_cat2: reader.read_i16_le()?,
//             instrument_cat3: reader.read_i16_le()?,
//             resource_container_filename: reader.read_i32_le()?,
//             wallpaper_filename: reader.read_i32_le()?,
//         })
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use std::fs::File;
//
//     use crate::Error;
//
//     use super::*;
//
//     #[test]
//     fn test_pubdata_0x28_0x80() -> Result<(), Error> {
//         let mut file = File::open("tests/patchdata/KontaktV42/ProgramData/0x28-0x80")?;
//         println!("{:?}", ProgramDataV80::read(&mut file)?);
//         Ok(())
//     }
// }
