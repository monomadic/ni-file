use crate::nks::BPatchMetaInfoHeader;

#[derive(Debug)]
pub struct Kon2 {
    pub zlib_length: u32,
    pub decompressed_length: u32,
    pub compressed_data: Vec<u8>,
    pub meta_info: BPatchMetaInfoHeader,
}

impl Kon2 {
    // pub fn resources();
    // pub fn preset();
}
