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
    #[br(parse_with = read_data_chunk)]
    pub data_chunk: DataChunk,

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

fn read_data_chunk<R: Read + Seek>(
    reader: &mut R,
    _ro: &binread::ReadOptions,
    _: (),
) -> BinResult<DataChunk> {
    let length: u64 = reader.read_le()?;

    let mut segment = vec![0; length as usize - 8];
    reader.read_exact(&mut segment)?;

    let mut cursor = Cursor::new(segment);

    let mut fields = Vec::new();

    loop {
        let dsin: DataField = cursor.read_le()?;

        if dsin.terminated != 0 {
            break;
        }

        fields.push(match dsin.type_id {
            118 => NIData::MainHeader(118),
            _ => NIData::Unknown(dsin.type_id),
        });
    }

    println!("data fields: {:?}", fields);

    Ok(DataChunk {
        fields
    })
}

#[derive(Debug)]
pub enum NIData {
    MainHeader(u32),
    Unknown(u32),
}

#[derive(Debug)]
pub struct DataChunk {
    // pub length: u64,
    pub fields: Vec<NIData>,
}

#[derive(BinRead, Debug)]
pub struct DataField {
    #[br(map = |val: [u8; 4]| String::from_utf8_lossy(&val).to_string())]
    pub tag: String,
    pub type_id: u32,
    pub unknown_a: u32, // always 1
    pub inner_length: u32, // could be 1
    pub terminated: u32, // 1 = end, 0 = read data
}
