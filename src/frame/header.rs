use crate::read_bytes::ReadBytesExt;

#[derive(Debug, Clone)]
pub struct FrameHeader {
    pub item_id: u32,      // (+0x8, uint)
    pub domain_id: u32,    // (+0xC, uint, 'hsin')
    pub header_flags: u32, // (0x10, uint)
    pub uuid: Vec<u8>,     // (0x14, int32_t)
}

impl FrameHeader {
    pub fn read<R>(mut reader: R) -> Result<FrameHeader, Box<dyn std::error::Error>>
    where
        R: ReadBytesExt,
    {
        // TODO: add runtime checks
        Ok(Self {
            item_id: reader.read_u32_le()?,
            domain_id: reader.read_u32_le()?,
            header_flags: reader.read_u32_le()?,
            uuid: reader.read_bytes(16)?,
        })
    }
}
