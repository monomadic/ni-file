use crate::{
    detect::NIFileType, file_container::NIFileContainer, nis::ItemContainer,
    nks::container::NKSContainer, read_bytes::*, Error,
};

pub enum NIFile {
    NKSContainer(NKSContainer),
    NISoundContainer(ItemContainer),
    Monolith(NIFileContainer),
    KontaktResource,
    NICompressedWave,
    NICache,
    FM8Preset,
}

pub enum NIPreset {
    KontaktInstrument,
}

impl NIFile {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let filetype = NIFileType::read(&mut reader)?;
        reader.rewind()?;

        Ok(match filetype {
            NIFileType::NISContainer => NIFile::NISoundContainer(ItemContainer::read(reader)?),
            NIFileType::Monolith => NIFile::Monolith(NIFileContainer::read(reader)?),
            NIFileType::NICompressedWave => NIFile::NICompressedWave,
            NIFileType::KoreSound => todo!(),
            NIFileType::NKSContainer(_) | NIFileType::KontaktMultiV1 => {
                NIFile::NKSContainer(NKSContainer::read(reader)?)
            }
            NIFileType::KontaktResource => NIFile::KontaktResource,
            NIFileType::KontaktCache => todo!(),
            NIFileType::NKSArchive => todo!(),
            NIFileType::NICache => NIFile::NICache,
            NIFileType::FM8LE => NIFile::FM8Preset,

            _ => todo!("Unsupported: {:?}", filetype),
        })
    }

    /// Extract raw preset data from this container (if applicable).
    pub fn inner_preset(&self) -> Result<Vec<u8>, Error> {
        match self {
            Self::NKSContainer(_nks) => unimplemented!(),
            Self::NISoundContainer(_nis) => unimplemented!(),
            _ => panic!("No preset detected."),
        }
    }
}
