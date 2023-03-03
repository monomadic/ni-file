#[derive(Debug, PartialEq)]
pub enum NIFileType {
    /// Most NI files are NIContainers.
    NIContainer,
    /// Kontakt files with samples inside are monoliths.
    NIKontaktMonolith,
    /// Kore has its own simple format.
    KoreSound,
    /// Not entirely sure if this is just k2 or all NI formats in the 90s
    Kontakt2,
    Unknown,
}

impl NIFileType {
    pub fn detect(buffer: &[u8]) -> NIFileType {
        filetype(buffer)
    }
}

pub fn filetype(buffer: &[u8]) -> NIFileType {
    // check for 'hsin' at byte 12
    if buffer[12..16] == [104, 115, 105, 110] {
        info!("Detected: NIContainer");
        return NIFileType::NIContainer;
    }

    // check for '/\ NI FC MTD  /\'
    if buffer[0..4] == [0x2F, 0x5C, 0x20, 0x4E] {
        info!("Detected: NIKontaktMonolith");
        return NIFileType::NIKontaktMonolith;
    }

    if buffer[0..4] == [0x12, 0x90, 0xA8, 0x7F] {
        info!("Detected: Kontakt2");
        return NIFileType::Kontakt2;
    }

    // check for '-ni-' at byte 0
    if buffer[0..4] == [45, 110, 105, 45] {
        info!("Detected: KoreSound");
        return NIFileType::KoreSound;
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
