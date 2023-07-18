use crate::{read_bytes::ReadBytesExt, Error};

/// Supported NI filetypes.
#[derive(Debug, PartialEq)]
pub enum NIFileType {
    /// All presets created after Kontakt5 are generally [`NISound`](NIFileType::NISound) containers.
    NISound,
    /// Kontakt archives with samples inside are [`NIMonolith`](crate::NIMonolith) containers.
    NIMonolith,
    /// Generally .ncw files created with Kontakt
    NICompressedWave,
    /// Kore has its own simple format.
    KoreSound,

    /// Kontakt instruments
    Kontakt1,
    NKS,

    KontaktResource,
    KontaktCache,

    Unknown,
}

//*
// other magic numbers:
// 0x464d3845   'FM8E'

impl NIFileType {
    /// Scan a buffer for magic numbers to detect NI filetypes.
    ///
    /// ```
    /// use ni_file::NIFileType;
    ///
    /// let file = std::fs::read("tests/data/kontakt-1/000-crunchy.nki").unwrap();
    ///
    /// if NIFileType::detect(&file) == NIFileType::NISound {
    ///     println!("NISound detected!");
    /// }
    /// ```
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let header_signature = reader.read_u32_le()?;

        Ok(match header_signature {
            0xB36EE55E => NIFileType::Kontakt1,

            0x7fa89012 | 0x10874353 | 0xab85ef01 => {
                info!("Detected: NKS (Little Endian)");
                NIFileType::NKS
            }
            0x1290A87F => NIFileType::NKS, // BE
            0xA4D6E55A | 0x74B5A69B => {
                panic!("kontakt: unknown");
            }
            0x54AC705E => NIFileType::KontaktResource,
            0x7A10E13F => NIFileType::KontaktCache,
            _ => {
                // check for 'hsin' at byte 12
                // TODO: NISound::detect()
                let _ = reader.read_u32_le()?;
                let hsin = reader.read_bytes(4)?;
                let hsin = hsin.as_slice();

                match hsin {
                    b"HSIN" => NIFileType::NISound,
                    [0x2F, 0x5C, 0x20, 0x4E] => NIFileType::NIMonolith,
                    [0x01, 0xA8, 0x9E, 0xD6] => NIFileType::NICompressedWave,
                    [45, 110, 105, 45] => NIFileType::KoreSound,
                    _ => NIFileType::Unknown,
                }
                //
                // if hsin == b"HSIN" {
                //     info!("Detected: NISound");
                //     return Ok(NIFileType::NISound);
                // }
                //
                // // BE monolith byte 35: 0x4916e63c
                // // 0x16ccf80a : valid sample magic?
                //
                // // .nkm
                // // check for '/\ NI FC MTD  /\' (NI FileContainer Metadata)
                // if hsin == [0x2F, 0x5C, 0x20, 0x4E] {
                //     info!("Detected: NIMonolith");
                //     return Ok(NIFileType::NIMonolith);
                // }
                //
                // // .ncw
                // if hsin == [0x01, 0xA8, 0x9E, 0xD6] {
                //     info!("Detected: NICompressedWave");
                //     return Ok(NIFileType::NICompressedWave);
                // }
                //
                // // check for '-ni-' at byte 0
                // if hsin == [45, 110, 105, 45] {
                //     info!("Detected: KoreSound");
                //     return NIFileType::KoreSound;
                // }
                //
                // // if buffer[0..4] == b"E8MF".to_owned() {
                // //     info!("Detected: FM8 Preset");
                // //     return NIFileType::FM8Preset;
                // // }
                //
                // error!("Unknown or unsupported filetype!");
                // NIFileType::Unknown
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kontakt_1() {
        assert_eq!(
            NIFileType::read(include_bytes!("../tests/data/kontakt-1/000-crunchy.nki").as_slice())
                .unwrap(),
            NIFileType::Kontakt1
        );
    }

    #[test]
    fn test_kontakt_7() {
        assert_eq!(
            NIFileType::read(
                include_bytes!("../tests/data/nisound/file/kontakt/7.1.3.0/000-default.nki")
                    .as_slice()
            )
            .unwrap(),
            NIFileType::NISound
        );
    }
}
