use crate::{
    kontakt::{
        chunk_set::KontaktChunks,
        objects::{NKIAppVersion, PatchType},
    },
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
        _version: &NKIAppVersion,
    ) -> Result<KontaktPreset, Error> {
        Ok(match patch_type {
            PatchType::NKM => Self::NKM(KontaktMulti::read(reader)?),
            PatchType::NKI => {
                match id {
                    "Kon4" => Self::KontaktV42(KontaktV42::read(reader)?),
                    "Kon5" | "Bat4" => Self::Kon5(Kon5::read(reader)?),
                    "Kon6" => Self::Kon6(Kon6::read(reader)?),
                    "Kon7" => Self::Kon7(Kon7::read(reader)?),
                    _ => unimplemented!(),
                }
                // // 4.2.0.0+
                // if version.major < 4 && version.minor_1 < 2 {
                //     todo!();
                // }
                //
                // // 5.1+
                // if version.major <= 5 && version.minor_1 <= 1 {
                //     dbg!(version);
                //     return Ok(Self::KontaktV42(KontaktV42::read(reader)?));
                // }
                //
                // return Ok(Self::Unsupported(KontaktChunks::read(reader)?));
            }
            _ => todo!(),
        })
    }
}
