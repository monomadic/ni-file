use crate::read_bytes::ReadBytesExt;

#[derive(Debug, PartialEq)]
pub enum NIFileType {
    /// Most NI files are NISound containers.
    NISound,
    /// Kontakt files with samples inside are monoliths.
    NIKontaktMonolith,
    NICompressedWave,
    /// Kore has its own simple format.
    KoreSound,

    /// Kontakt instruments
    Kontakt1,
    Kontakt2,
    Kontakt42,

    NIPresetKontakt,

    FM8Preset,
    Unknown,
}

//*
// other magic numbers:
// 0x464d3845   'FM8E'

impl NIFileType {
    pub fn detect(buffer: &[u8]) -> NIFileType {
        filetype(buffer)
    }
}

pub fn filetype(buffer: &[u8]) -> NIFileType {
    let mut reader = buffer.clone();
    let header_signature = reader.read_u32_le().unwrap();

    match header_signature {
        0xB36EE55E => {
            info!("Detected: Kontakt1");
            NIFileType::Kontakt1
        }
        0x7fa89012 | 0x10874353 | 0xab85ef01 => {
            info!("Detected: Kontakt2 (Little Endian)");
            NIFileType::Kontakt2
        }
        0xa4d6e55a | 0x74b5a69b => {
            panic!("kontakt: unknown");
        }
        _ => {
            // .nki, .nfm8, etc
            // check for 'hsin' at byte 12
            // TODO: NISound::detect()
            if buffer[12..16] == [104, 115, 105, 110] {
                info!("Detected: NIContainer");
                return NIFileType::NISound;
            }

            // BE monolith byte 35: 0x4916e63c
            // 0x16ccf80a : valid sample magic?

            // .nkm
            // check for '/\ NI FC MTD  /\' (NI FileContainer Metadata)
            if buffer[0..4] == [0x2F, 0x5C, 0x20, 0x4E] {
                info!("Detected: NIKontaktMonolith");
                return NIFileType::NIKontaktMonolith;
            }

            // .ncw
            if buffer[0..4] == [0x01, 0xA8, 0x9E, 0xD6] {
                info!("Detected: NICompressedWave");
                return NIFileType::NICompressedWave;
            }

            // check for '-ni-' at byte 0
            if buffer[0..4] == [45, 110, 105, 45] {
                info!("Detected: KoreSound");
                return NIFileType::KoreSound;
            }

            if buffer[0..4] == b"E8MF".to_owned() {
                info!("Detected: FM8 Preset");
                return NIFileType::FM8Preset;
            }

            error!("Unknown or unsupported filetype!");
            NIFileType::Unknown
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kontakt_1() {
        assert_eq!(
            filetype(include_bytes!(
                "../tests/data/nisound/file/kontakt/000-crunchy.nki"
            )),
            NIFileType::Kontakt1
        );
    }

    #[test]
    fn test_kontakt_7() {
        assert_eq!(
            filetype(include_bytes!(
                "../tests/data/files/kontakt-7/000-default.nki"
            )),
            NIFileType::NISound
        );
    }
}
