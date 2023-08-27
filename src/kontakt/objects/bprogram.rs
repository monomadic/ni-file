use crate::{kontakt::structured_object::StructuredObject, read_bytes::ReadBytesExt, Error};
use std::io;

use super::program_data::ProgramDataV80;

#[derive(Debug)]
pub struct BProgram {
    // public: ProgramPublicParams,
    public: ProgramDataV80,
    private: ProgramPrivateParams,
}

impl BProgram {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let so = StructuredObject::read(&mut reader)?;

        Ok(Self {
            public: ProgramDataV80::read(io::Cursor::new(so.public_data))?,
            private: ProgramPrivateParams::read(io::Cursor::new(so.private_data), so.version)?,
        })
    }
}

impl TryFrom<StructuredObject> for BProgram {
    type Error = Error;

    fn try_from(so: StructuredObject) -> Result<Self, Self::Error> {
        Ok(Self {
            // TODO: support other versions
            public: ProgramDataV80::read(io::Cursor::new(so.public_data.as_slice()))?,
            private: ProgramPrivateParams::read(
                io::Cursor::new(so.private_data.as_slice()),
                so.version,
            )?,
        })
    }
}

#[derive(Debug, Default)]
pub struct ProgramPublicParams {
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

// impl ProgramPublicParams {
//     pub fn read<R: ReadBytesExt>(mut reader: R, version: u16) -> Result<Self, Error> {
//         match version {
//             0x80 => ProgramDataV80::read(&mut reader),
//             _ => todo!(),
//         }
//     }
// }

#[derive(Debug, Default)]
pub struct ProgramPrivateParams {}

impl ProgramPrivateParams {
    pub fn read<R: ReadBytesExt>(mut reader: R, version: u16) -> Result<Self, Error> {
        if version != 0x80 {
            panic!("unsupported version: {version}");
        }
        // assume v80 for now

        let version = reader.read_u32_le()?;
        println!("version {:?}", version);
        assert!(
            version < 2,
            "ProgramPrivateParams unsupported version: {version}"
        );

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

        // BFileName - SER::ReadBFNTrns
        // should be -1
        let filename = reader.read_i32_le()?;
        if filename > 0 {
            panic!("Unsupported: SER::ReadBFNTrns");
        }

        // 5 x SER::Read(stream,(BSerializable *)&this[0xcb].field_0x12d, ctx);

        // SER::ReadBHeapArr<>

        // SER::ARRAY::ReadInsert<>
        // SER::ARRAY::ReadInsert<>

        let filename = reader.read_i32_le()?;
        if filename > 0 {
            panic!("Unsupported: SER::ReadBFNTrns");
        }

        // another BFileName

        Ok(Self {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_private_params_v80() -> Result<(), Error> {
        let mut file = std::io::Cursor::new(include_bytes!(
            "../../../tests/patchdata/KontaktV42/Program/v80/private_params/000"
        ));
        let params = ProgramPrivateParams::read(&mut file, 0x80)?;
        Ok(())
    }
}
