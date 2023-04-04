use crate::read_bytes::ReadBytesExt;

#[derive(Debug, PartialEq)]
pub enum NIFileType {
    /// Most NI files are NIContainers.
    NIContainer,
    /// Kontakt files with samples inside are monoliths.
    NIKontaktMonolith,
    /// Kore has its own simple format.
    KoreSound,
    /// Not entirely sure if this is just k2 or all NI formats in the 90s
    Kontakt1,
    Kontakt2,

    NICompressedWave,

    FM8Preset,

    Unknown,
}

impl NIFileType {
    pub fn detect(buffer: &[u8]) -> NIFileType {
        filetype(buffer)
    }
}

pub fn filetype(buffer: &[u8]) -> NIFileType {
    let mut reader = buffer.clone();
    let header_signature = reader.read_u32_le().unwrap();
    match header_signature {
        0x7fa89012 => {}
        _ => (),
    }

    // .nki, .nfm8, etc
    // check for 'hsin' at byte 12
    if buffer[12..16] == [104, 115, 105, 110] {
        info!("Detected: NIContainer");
        return NIFileType::NIContainer;
    }

    // .nkm
    // check for '/\ NI FC MTD  /\' (NI FileContainer Metadata)
    if buffer[0..4] == [0x2F, 0x5C, 0x20, 0x4E] {
        info!("Detected: NIKontaktMonolith");
        return NIFileType::NIKontaktMonolith;
    }

    if buffer[0..4] == [0x5E, 0xE5, 0x6E, 0xB3] {
        info!("Detected: Kontakt1");
        return NIFileType::Kontakt1;
    }

    // .nki
    if buffer[0..4] == [0x12, 0x90, 0xA8, 0x7F] {
        info!("Detected: Kontakt2");
        return NIFileType::Kontakt2;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kontakt_7() {
        assert_eq!(
            filetype(include_bytes!(
                "../tests/data/files/kontakt-7/000-default.nki"
            )),
            NIFileType::NIContainer
        )
    }
}
