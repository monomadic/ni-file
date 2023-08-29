use std::io::Cursor;

use crate::{
    kontakt::{
        chunkdata::ChunkData, error::KontaktError, structured_object::StructuredObject,
        zone_data::ZoneData, zone_list::ZoneList,
    },
    read_bytes::ReadBytesExt,
    Error,
};

use super::program_data::ProgramDataV80;

#[derive(Debug)]
pub struct Program(StructuredObject);

impl Program {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        Ok(Self(StructuredObject::read(&mut reader)?))
    }

    pub fn public_params(&self) -> Result<ProgramDataV80, Error> {
        let reader = Cursor::new(&self.0.public_data);

        match self.0.version {
            0x80 => Ok(ProgramDataV80::read(reader)?),
            _ => todo!(),
        }
    }

    pub fn zones(&self) -> Option<Result<Vec<ZoneData>, Error>> {
        self.0
            .find_first(0x34)
            .map(|chunk| ZoneList::try_from(chunk).map(|z| z.zones))
    }

    // 0x32 VoiceGroups
    // 0x33 GroupList
}

impl std::convert::TryFrom<&ChunkData> for Program {
    type Error = Error;

    fn try_from(chunk: &ChunkData) -> Result<Self, Self::Error> {
        if chunk.id != 0x28 {
            return Err(KontaktError::IncorrectID {
                expected: 0x28,
                got: chunk.id,
            }
            .into());
        }
        let reader = Cursor::new(&chunk.data);
        Self::read(reader)
    }
}

#[derive(Debug, Default)]
pub struct ProgramDataPublicParams {
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

#[derive(Debug, Default)]
pub struct ProgramDataPrivateParams {}

impl ProgramDataPrivateParams {
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
    use std::fs::File;

    use super::*;

    #[test]
    fn test_private_params_v80() -> Result<(), Error> {
        let mut file = File::open("tests/patchdata/KontaktV42/Program/v80/private_params/000")?;
        let _params = ProgramDataPrivateParams::read(&mut file, 0x80)?;
        Ok(())
    }
}
