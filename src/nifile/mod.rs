use crate::{detect::NIFileType, nks::nksfile::NKSFile, read_bytes::*, Error, Repository};

pub enum NIFile {
    NKSContainer(NKSFile),
    NISContainer(Repository),
    Monolith,
    KontaktResource,
    NICompressedWave,
    NICache,
}

impl NIFile {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let filetype = NIFileType::read(&mut reader)?;
        reader.rewind()?;

        use NIFile::*;
        Ok(match filetype {
            NIFileType::NISContainer => NISContainer(Repository::read(reader)?),
            NIFileType::Monolith => Monolith,
            NIFileType::NICompressedWave => NICompressedWave,
            NIFileType::KoreSound => todo!(),
            NIFileType::KontaktInstrumentV1 => todo!(),
            NIFileType::NKSInstrument => NKSContainer(NKSFile::read(reader)?),
            NIFileType::KontaktResource => KontaktResource,
            NIFileType::KontaktCache => todo!(),
            NIFileType::NKSArchive => todo!(),
            NIFileType::NICache => NICache,

            _ => todo!("Unsupported: {:?}", filetype),
        })
    }
}
