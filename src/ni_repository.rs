// use crate::ni_segment::SegmentType;
use crate::Error;
use binread::{io::Cursor, prelude::*};

/// Native Instruments Container object
/// - represents an entire instrument file
#[derive(BinRead, Debug, Clone)]
pub struct Repository {
    pub header: ItemHeader,

    pub uuid: [u8; 16], // (0x14, int32_t)

    pub data: Data,

    pub unknown: u32,
    pub number_of_children: u32,

    #[br(count = number_of_children)]
    pub children: Vec<ItemFrame>,
}

#[derive(BinRead, Debug, Clone)]
pub struct Data {
    pub size: u64,

    #[br(count=size, seek_before=std::io::SeekFrom::Current(-8))]
    pub data: Vec<u8>,
}

#[derive(BinRead, Debug, Clone)]
pub struct ItemFrame {
    pub current_index: u32,
    pub domain_id: u32, // 'DSIN'
    pub item_id: u32,   // segment type
    pub repository: Repository,
}

impl Repository {
    pub fn read(buf: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buf);

        Ok(cursor.read_le()?)
    }
}

// TODO: check the offsets
#[derive(BinRead, Debug, Clone)]
pub struct ItemHeader {
    pub size: u64,         // (0x0, ulong)
    pub item_id: u32,      // (+0x8, uint)
    pub domain_id: u32,    // (+0xC, uint, 'hsin')
    pub header_flags: u32, // (0x10, uint)
    pub unknown: u32,
}

// // terminator item
// // afl~method.NI::SOUND::Item.
// pub struct Item {
//     pub domain_id: u32,   // 'DSIN' / 0x4e495344
//     pub item_id: u32,     // '1' constant
//     pub child_items: u32, // (+0x08, int32_t)
//     pub uuid: u32,        // (+0x10, uint)
// }

// 16 bytes
#[derive(BinRead, Debug)]
pub struct Uuid {
    pub a: u32,
    pub b: u16,
    pub c: u16,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub g: u8,
    pub h: u8,
    pub i: u8,
    pub j: u8,
    pub k: u8,
}
