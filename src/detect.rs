use crate::{nis::ItemContainer, nks::detect::NKSFileType, read_bytes::ReadBytesExt, Error};

/// Supported NI filetypes.
#[derive(Debug, PartialEq)]
pub enum NIFileType {
    /// Kontakt Sound Container
    NKSContainer(NKSFileType),
    /// Native Instruments Sound Container
    NISContainer,
    /// Monolith/FileContainer Container
    Monolith,
    /// Losslessly compressed audio.
    NICompressedWave,
    /// Kore has its own simple format.
    KoreSound,
    /// Kontakt instruments
    KontaktMultiV1,
    NKSArchive,
    NICache,
    KontaktResource,
    KontaktCache,
    Unknown,
    FM8LE,
}

impl NIFileType {
    /// Detect NI filetypes based on file signatures.
    ///
    /// ```
    /// use ni_file::NIFileType;
    ///
    /// let file = File::open("tests/data/kontakt-1/000-crunchy.nki").unwrap();
    ///
    /// if NIFileType::detect(&file) == NIFileType::NISound {
    ///     println!("NISound detected!");
    /// }
    /// ```
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let magic: u32 = reader.read_le()?;

        if let Some(nks) = NKSFileType::detect(magic) {
            return Ok(NIFileType::NKSContainer(nks));
        }

        // TODO: differentiate LE/BE
        Ok(match magic {
            0x5AE5D6A4 | 0xA4D6E55A => NIFileType::KontaktMultiV1,
            0x54AC705E | 0x5E70AC54 => NIFileType::KontaktResource,
            0x4916E63C | 0x3CE61649 => NIFileType::NKSArchive,
            0x01A89ED6 | 0xD69EA801 => NIFileType::NICompressedWave,
            0x7A10E13F | 0x3FE1107A => NIFileType::NICache,
            0x2F5C204E | 0x4E205C2F => NIFileType::Monolith,
            0x464D3845 => NIFileType::FM8LE, // "FM8E"
            _ => {
                reader.rewind()?;
                match ItemContainer::read(&mut reader) {
                    Ok(_) => NIFileType::NISContainer,
                    Err(_) => NIFileType::Unknown,
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_kontakt_1() -> Result<(), Error> {
        Ok(assert_eq!(
            NIFileType::read(File::open("tests/filetype/NKS/KontaktV1/000-crunchy.nki")?)?,
            NIFileType::NKSContainer(NKSFileType::NKSv1LE)
        ))
    }

    #[test]
    fn test_kontakt_nkm() -> Result<(), Error> {
        Ok(assert_eq!(
            NIFileType::read(File::open("test-data/NKM/000.nkm")?)?,
            NIFileType::KontaktMultiV1
        ))
    }

    #[test]
    fn test_kontakt_2() -> Result<(), Error> {
        Ok(assert_eq!(
            NIFileType::read(File::open(
                "tests/filetype/NKS/KontaktV2/KontaktV2-000.nki"
            )?)?,
            NIFileType::NKSContainer(NKSFileType::NKSv2LE)
        ))
    }

    #[test]
    fn test_kontakt_42() -> Result<(), Error> {
        Ok(assert_eq!(
            NIFileType::read(File::open(
                "tests/filetype/NKS/KontaktV42/KontaktV42-000.nki"
            )?)?,
            NIFileType::NKSContainer(NKSFileType::NKSv2LE)
        ))
    }

    #[test]
    fn test_ncw() -> Result<(), Error> {
        Ok(assert_eq!(
            NIFileType::read(File::open("test-data/NCW/16-bit.ncw")?)?,
            NIFileType::NICompressedWave
        ))
    }

    #[test]
    fn test_kontakt_7() -> Result<(), Error> {
        let file = File::open("tests/filetype/NISD/kontakt/7.1.3.0/000-default.nki")?;
        Ok(assert_eq!(
            NIFileType::read(file)?,
            NIFileType::NISContainer
        ))
    }
}
