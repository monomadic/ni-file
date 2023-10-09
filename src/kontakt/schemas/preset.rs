use crate::{read_bytes::ReadBytesExt, Error};

use super::{kon3::Kon3, kon5::Kon5, kon6::Kon6, kon7::Kon7, Kon1, Kon2, Kon4};

#[derive(Debug)]
pub enum KontaktPreset {
    Kon1(Kon1),
    Kon2(Kon2),
    Kon3(Kon3),
    Kon4(Kon4),
    Kon5(Kon5),
    Kon6(Kon6),
    Kon7(Kon7),
}

impl KontaktPreset {
    pub fn from_str<R: ReadBytesExt>(reader: &mut R, id: &str) -> Result<KontaktPreset, Error> {
        Ok(match id {
            "Kon1" => Self::Kon1(Kon1::read(reader)?),
            "Kon2" => Self::Kon2(Kon2::read(reader)?),
            "Kon3" => Self::Kon3(Kon3::read(reader)?),
            "Kon4" => Self::Kon4(Kon4::read(reader)?),
            "Kon5" => Self::Kon5(Kon5::read(reader)?),
            "Kon6" => Self::Kon6(Kon6::read(reader)?),
            "Kon7" => Self::Kon7(Kon7::read(reader)?),
            _ => unimplemented!("{}", id),
        })
    }
}
