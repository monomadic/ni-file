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
    pub unknown_a: u32, // always 1
    //#[br(magic = b"DSIN", assert(id==108))]
    pub tag: [char; 4],
    pub id: u64,
    pub checksum: [u8;16], // still unknown, maybe md4?
    pub data_len: u32,
    #[br(count = data_len - 4)]
    pub data_chunk: Vec<u8>,
    pub unknown_b: u32, // always 1?
    pub children: u64,
    pub inner_tag: [char; 4],
    pub inner_id: u32,
    pub inner_length: u64,
    #[br(count = inner_length - 8)]
    pub inner_chunk: Vec<u8>,
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

#[derive(BinRead, Debug)]
struct DataChunk {
    size: u32,
    unknown_a: u32,
    tag: [char; 4],
    id: u32,
    unknown_b: u32,
}
