use std::io::Cursor;

use crate::{
    kontakt::{chunk::Chunk, error::KontaktError, structured_object::StructuredObject},
    read_bytes::ReadBytesExt,
    Error,
};

use super::zone_list::ZoneList;

const CHUNK_ID: u16 = 0x28;

/// SerType:        0x28
/// Known Versions: 0x80 .. 0xAF
/// Kontakt 7:      BProgram
/// KontaktIO:      K4PL_Program
#[derive(Debug)]
pub struct Program(pub StructuredObject);

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
    pub fn read<R: ReadBytesExt>(mut reader: R, version: u16) -> Result<Self, Error> {
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

impl Program {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        Ok(Self(StructuredObject::read(&mut reader)?))
    }

    pub fn version(&self) -> u16 {
        self.0.version
    }

    pub fn params(&self) -> Result<ProgramPublicParams, Error> {
        let reader = Cursor::new(&self.0.public_data);

        ProgramPublicParams::read(reader, self.0.version)

        // match self.0.version {
        //     0x80 | 0x82 | 0x90 => Ok(ProgramDataV80::read(reader)?),
        //     0xa6 => todo!(),
        //     0xa7 => todo!(),
        //     0xa8 | 0xa9 | 0xaa | 0xab | 0xac | 0xad | 0xae => todo!(),
        //     0xaf => todo!(),
        //     _ => todo!(),
        // }
    }

    pub fn zone_list(&self) -> Option<Result<ZoneList, Error>> {
        self.0
            .find_first(0x34)
            .map(|chunk| ZoneList::try_from(chunk))
    }

    pub fn children(&self) -> &Vec<Chunk> {
        &self.0.children
    }

    // 0x32 VoiceGroups
    // 0x33 GroupList
}

impl std::convert::TryFrom<&Chunk> for Program {
    type Error = Error;

    fn try_from(chunk: &Chunk) -> Result<Self, Self::Error> {
        if chunk.id != CHUNK_ID {
            return Err(KontaktError::IncorrectID {
                expected: CHUNK_ID,
                got: chunk.id,
            }
            .into());
        }
        let reader = Cursor::new(&chunk.data);
        Self::read(reader)
    }
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
        let mut file = File::open("tests/data/Objects/Kontakt/Program/v80/private_params/000")?;
        let _params = ProgramDataPrivateParams::read(&mut file, 0x80)?;
        Ok(())
    }
}
