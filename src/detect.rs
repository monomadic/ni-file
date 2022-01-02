pub enum NIFileType {
    NIContainer,
    NIKontaktMonolith,
    KoreSound,
    Unknown,
}

pub(crate) fn filetype(buffer: &[u8]) -> NIFileType {
    // check for 'hsin' at byte 12
    if buffer[12..16] == [104, 115, 105, 110] {
        return NIFileType::NIContainer;
    }

    // check for '/\ NI FC MTD  /\'
    if buffer[0..4] == [0x2F, 0x5C, 0x20, 0x4E] {
        return NIFileType::NIKontaktMonolith;
    }

    // check for '-ni-' at byte 0
    if buffer[0..4] == [45, 110, 105, 45] {
        return NIFileType::KoreSound;
    }

    NIFileType::Unknown
}