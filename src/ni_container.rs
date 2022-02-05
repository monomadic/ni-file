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
    pub length: u32,
    pub unknown_a: u32, // always 0
    pub unknown_b: u32,
    //#[br(magic = b"DSIN", assert(id==108))]
    pub tag: [char;4],
    pub id: u32,
    pub unknown_c: u32,
    pub checksum: [u32;4], // still unknown, maybe md4?
}

impl HeaderChunk {
    pub fn tag(&self) -> String {
        self.tag.iter().collect::<String>()
    }
}

impl ToString for HeaderChunk {
    fn to_string(&self) -> std::string::String { 
        format!("<{}>", &self.tag())
    }
}
