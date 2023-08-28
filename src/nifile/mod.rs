use crate::{nks::nksfile::NKSFile, read_bytes::*, Error, NIFileType, Repository};

pub enum NIFile {
    NKSContainer(NKSFile),
    NISContainer(Repository),
    FileContainer,
}

impl NIFile {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let filetype = NIFileType::detect(&mut reader)?;
        reader.rewind()?;

        use NIFile::*;
        Ok(match filetype {
            NIFileType::NISContainer => NISContainer(Repository::read(reader)?),
            NIFileType::FileContainer => FileContainer,
            NIFileType::NICompressedWave => todo!(),
            NIFileType::KoreSound => todo!(),
            NIFileType::Kontakt1 => todo!(),
            NIFileType::NKSContainer => NKSContainer(NKSFile::read(reader)?),
            NIFileType::KontaktResource => todo!(),
            NIFileType::KontaktCache => todo!(),
            NIFileType::Unknown => todo!(),
        })
    }
}
