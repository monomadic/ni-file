use crate::Error;
use binread::{io::Cursor, prelude::*};
use std::io::prelude::*;
use crate::ni_segment::NIData;

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
    #[br(parse_with = read_data_frames)]
    pub data_chunk: DataField,

    pub current_index: u32,
    pub children_length: u32,

    #[br(count = children_length)]
    pub children: Vec<ChildChunk>,
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

fn read_data_frames<R: Read + Seek>(reader: &mut R, ro: &binread::ReadOptions, _: (),) -> BinResult<DataField> {
    let dsin: DataFieldHeader = reader.read_le()?;

    let mut data = vec![0; dsin.length as usize - 20];
    reader.read_exact(&mut data)?;

    let mut data_cursor = Cursor::new(&data);

    let child: Option<Box<DataField>> = match dsin.type_id {
        1 => None,
        _ => Some(Box::new(read_data_frames(&mut data_cursor, ro, ())?)),
    };

    let mut data_bin = Vec::new();
    data_cursor.read_to_end(&mut data_bin)?;
    let data = NIData::read(dsin.type_id, &data_bin)?;

    Ok(DataField {
        tag: dsin.tag,
        type_id: dsin.type_id,
        unknown_a: dsin.unknown_a,
        data,
        child,
    })
}

#[derive(Debug)]
pub struct DataField {
    pub tag: String,
    pub type_id: u32,
    pub unknown_a: u32, // always 1
    pub data: NIData,
    pub child: Option<Box<DataField>>,
}

#[derive(BinRead, Debug)]
pub struct DataFieldHeader {
    pub length: u64,
    #[br(map = |val: [u8; 4]| String::from_utf8_lossy(&val).to_string())]
    pub tag: String,
    pub type_id: u32,
    pub unknown_a: u32, // always 1
}