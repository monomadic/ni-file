mod kontakt;

use crate::{nks::nksfile::NKSFile, read_bytes::*, Error, NIFileType};

pub enum NIFile {
    Kontakt(NKSFile),
}

impl NIFile {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let cursor = reader.by_ref();
        match NIFileType::detect(cursor)? {
            NIFileType::NISound => todo!(),
            NIFileType::FileContainer => todo!(),
            NIFileType::NICompressedWave => todo!(),
            NIFileType::KoreSound => todo!(),
            NIFileType::Kontakt1 => todo!(),
            NIFileType::NKS => Ok(NIFile::Kontakt(NKSFile::read(&mut reader)?)),
            NIFileType::KontaktResource => todo!(),
            NIFileType::KontaktCache => todo!(),
            NIFileType::Unknown => todo!(),
        }
    }
}
