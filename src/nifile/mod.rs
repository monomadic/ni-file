use crate::{
    detect::NIFileType, file_container::NIFileContainer, nks::container::NKSContainer,
    read_bytes::*, Error, Repository,
};

pub enum NIFile {
    NKSContainer(NKSContainer),
    NISContainer(Repository),
    Monolith(NIFileContainer),
    KontaktResource,
    NICompressedWave,
    NICache,
}

impl NIFile {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let filetype = NIFileType::read(&mut reader)?;
        reader.rewind()?;

        Ok(match filetype {
            NIFileType::KontaktInstrumentV1 => NIFile::NKSContainer(NKSContainer::read(reader)?),
            NIFileType::NISContainer => NIFile::NISContainer(Repository::read(reader)?),
            NIFileType::Monolith => NIFile::Monolith(NIFileContainer::read(reader)?),
            NIFileType::NICompressedWave => NIFile::NICompressedWave,
            NIFileType::KoreSound => todo!(),
            NIFileType::NKSInstrument => NIFile::NKSContainer(NKSContainer::read(reader)?),
            NIFileType::KontaktResource => NIFile::KontaktResource,
            NIFileType::KontaktCache => todo!(),
            NIFileType::NKSArchive => todo!(),
            NIFileType::NICache => NIFile::NICache,

            _ => todo!("Unsupported: {:?}", filetype),
        })
    }
}
