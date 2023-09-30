/// Supported NKS filetypes.
#[derive(Debug, PartialEq)]
pub enum NKSFileType {
    NKSv1LE,
    NKSv1BE,
    NKSv2LE,
    NKSv2BE,
}

impl NKSFileType {
    pub fn detect(magic: u32) -> Option<NKSFileType> {
        match magic {
            0x5EE56EB3 => Some(NKSFileType::NKSv1BE),
            0xB36EE55E => Some(NKSFileType::NKSv1LE),
            0x1290A87F => Some(NKSFileType::NKSv2BE),
            0x7FA89012 => Some(NKSFileType::NKSv2LE),
            _ => None,
        }
    }
}
