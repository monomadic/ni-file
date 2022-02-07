use crate::Error;
use binread::{io::Cursor, prelude::*, NullWideString};
use std::io::prelude::*;

pub fn read(buf: &[u8]) -> Result<HeaderChunk, Error> {
    let mut cursor = Cursor::new(buf);
    let segment: HeaderChunk = cursor.read_le()?;
    Ok(segment)
}

#[derive(BinRead, Debug)]
pub struct HeaderChunk {
    pub length: u64,
    pub unknown_a: u32,
    #[br(assert(tag==['h','s','i','n']))]
    pub tag: [char; 4],
    pub id: u64,
    pub checksum: [u8;16], // md5 of child section (including child chunk)

    pub data_len: u32,
    #[br(count = data_len, seek_before=std::io::SeekFrom::Current(-4))]
    pub data_chunk: Vec<u8>,

    pub current_index: u32,
    pub children_length: u32,

    #[br(count = children_length)]
    pub children: Vec<ChildChunk>,

    // pub inner_length: u64,
    // #[br(count = inner_length, seek_before=std::io::SeekFrom::Current(-8))]
    // pub inner_chunk: Vec<u8>,
}

#[derive(BinRead, Debug)]
pub struct ChildChunk {
    pub unknown_a: u32, // SUSPICIOUS NUMBER - COMPRESSION HAS HIGH VALUE
    pub tag: [char; 4],
    pub id: u32,
    pub chunk: HeaderChunk,
}

impl HeaderChunk {
    pub fn tag(&self) -> String {
        self.tag.iter().collect::<String>()
    }
    pub fn checksum(&self) -> String {
        self.checksum
            .iter()
            .map(|h| format!("{:02x}", h))
            .collect::<String>()
    }
}

impl ToString for HeaderChunk {
    fn to_string(&self) -> std::string::String {
        format!("<{}></{}>", &self.tag(), &self.tag())
    }
}

/// for testing: collects child data as bin
#[derive(BinRead, Debug)]
pub struct HeaderChunkDump {
    pub length: u64,
    pub unknown_a: u32, // always 1
    // #[br(assert(tag==['h','s','i','n']))]
    pub tag: [char; 4],
    pub id: u64,
    pub checksum: [u8;16], // md5 of child section (including child chunk)

    pub data_len: u32,
    #[br(count = data_len, seek_before=std::io::SeekFrom::Current(-4))]
    pub data_chunk: Vec<u8>,

    pub unknown_b: u32, // always 1? index?
    pub children: u32,

    // #[br(count = children)]
    // pub inner: Vec<ChildChunkDump>,
}

#[derive(BinRead, Debug)]
pub struct ChildChunkDump {
    pub unknown_a: u32, // VERY SUSPICIOUS NUMBER
    pub tag: [char; 4],
    pub id: u32,

    pub inner_length: u64,
    // #[br(count = inner_length, seek_before=std::io::SeekFrom::Current(-8))]
    // pub inner_chunk: Vec<u8>,
}

#[derive(BinRead, Debug)]
pub struct DataSection {
    pub length: u64,
    pub data: Vec<u8>,
}

#[derive(BinRead, Debug)]
pub struct DataTag {
    pub tag: [char; 4],
    pub type_id: u32,
    pub unknown_a: u32, // always 1
    pub inner_length: u32, // could be 1
    pub terminated: u32, // 1 = end, 0 = read data
    pub remaining_data: Vec<u8>,
}