use std::io::Cursor;

use crate::{
    kontakt::{chunk::Chunk, error::KontaktError, structured_object::StructuredObject},
    read_bytes::ReadBytesExt,
    Error,
};

use super::{program_data::ProgramPublicParams, zone_data::ZoneData, zone_list::ZoneList};

/// SerType:        0x28
/// Known Versions: 0x80 .. 0xAF
/// Kontakt 7:      BProgram
/// KontaktIO:      K4PL_Program
#[derive(Debug)]
pub struct Program(StructuredObject);

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

    pub fn zones(&self) -> Option<Result<Vec<ZoneData>, Error>> {
        self.0
            .find_first(0x34)
            .map(|chunk| ZoneList::try_from(chunk).map(|z| z.zones))
    }

    pub fn children(&self) -> &Vec<Chunk> {
        &self.0.children
    }

    // 0x32 VoiceGroups
    // 0x33 GroupList
}

// impl std::convert::TryFrom<Chunk> for Program {
//     type Error = KontaktError;
//
//     fn try_from(chunk: Chunk) -> Result<Self, Self::Error> {
//         if chunk.id != 0x28 {
//             return Err(KontaktError::IncorrectID {
//                 expected: 0x28,
//                 got: chunk.id,
//             }
//             .into());
//         }
//         let reader = Cursor::new(&chunk.data);
//         Self::read(reader)
//     }
// }

impl std::convert::TryFrom<&Chunk> for Program {
    type Error = Error;

    fn try_from(chunk: &Chunk) -> Result<Self, Self::Error> {
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
