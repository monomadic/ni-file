use binread::BinRead;

#[derive(BinRead, Debug, Clone)]
struct FrameHeader {
    pub item_id: u32,      // (+0x8, uint)
    pub domain_id: u32,    // (+0xC, uint, 'hsin')
    pub header_flags: u32, // (0x10, uint)
    pub uuid: [u8; 16],    // (0x14, int32_t)
}
