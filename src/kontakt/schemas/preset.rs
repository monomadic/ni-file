use crate::{
    kontakt::{chunk_set::KontaktChunks, objects::PatchType},
    read_bytes::ReadBytesExt,
    Error,
};

use super::{
    kon5::Kon5, kon6::Kon6, kon7::Kon7, multi::KontaktMulti, KontaktV1, KontaktV2, KontaktV42,
};

#[derive(Debug)]
pub enum KontaktPreset {
    KontaktV1(KontaktV1),
    KontaktV2(KontaktV2),
    KontaktV42(KontaktV42),
    Kon5(Kon5),
    Kon6(Kon6),
    Kon7(Kon7),
    NKM(KontaktMulti),
    // NKI(KontaktInstrument),
    Unsupported(KontaktChunks),
}

impl KontaktPreset {
    pub fn read<R: ReadBytesExt>(
        reader: R,
        id: &str,
        patch_type: &PatchType,
    ) -> Result<KontaktPreset, Error> {
        Ok(match patch_type {
            PatchType::NKM => Self::NKM(KontaktMulti::read(reader)?),
            PatchType::NKI => match id {
                "Kon4" => Self::KontaktV42(KontaktV42::read(reader)?),
                "Bat4" | "Kon5" => Self::Kon5(Kon5::read(reader)?),
                "Kon6" => Self::Kon6(Kon6::read(reader)?),
                "Kon7" => Self::Kon7(Kon7::read(reader)?),
                _ => Self::Unsupported(KontaktChunks::read(reader)?),
            },
            _ => todo!(),
        })
    }
}
