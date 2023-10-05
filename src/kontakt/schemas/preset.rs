use crate::{read_bytes::ReadBytesExt, Error};

use super::{kon7::Kon7, Kon1, Kon4, XMLDocument};

#[derive(Debug)]
pub enum KontaktPreset {
    Kon1(Kon1),
    Kon2(XMLDocument),
    Kon3(XMLDocument),
    Kon4(Kon4),
    Kon7(Kon7),
}

impl KontaktPreset {
    pub fn from_str<R: ReadBytesExt>(reader: &mut R, id: &str) -> Result<KontaktPreset, Error> {
        Ok(match id {
            "Kon1" => Self::Kon1(Kon1::read(reader)?),
            "Kon4" => Self::Kon4(Kon4::read(reader)?),
            "Kon7" => Self::Kon7(Kon7::read(reader)?),
            _ => unimplemented!("{}", id),
        })
    }
}
