use crate::{read_bytes::ReadBytesExt, Error};

use super::{kon5::Kon5, kon6::Kon6, kon7::Kon7, KontaktV1, KontaktV2, KontaktV42};

#[derive(Debug)]
pub enum KontaktPreset {
    KontaktV1(KontaktV1),
    KontaktV2(KontaktV2),
    KontaktV42(KontaktV42),
    Kon5(Kon5),
    Kon6(Kon6),
    Kon7(Kon7),
}

impl KontaktPreset {
    pub fn from_fourcc<R: ReadBytesExt>(reader: &mut R, id: &str) -> Result<KontaktPreset, Error> {
        Ok(match id {
            "Kon4" => Self::KontaktV42(KontaktV42::read(reader)?),
            "Kon5" => Self::Kon5(Kon5::read(reader)?),
            "Kon6" => Self::Kon6(Kon6::read(reader)?),
            "Kon7" => Self::Kon7(Kon7::read(reader)?),
            _ => unimplemented!("{}", id),
        })
    }
}
