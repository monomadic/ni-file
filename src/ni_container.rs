use crate::ni_segment::SegmentType;
use crate::Error;
use binread::{io::Cursor, prelude::*};
use byteorder::ReadBytesExt;
use std::io::prelude::*;

/// Native Instruments Container object
/// - represents an entire instrument file
pub struct NIContainer {
    pub data: HeaderChunk,
}
impl NIContainer {
    pub fn read(buf: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buf);

        Ok(Self {
            data: cursor.read_le()?,
        })
    }

    pub fn to_xml(&self) -> String {
        format!("{}\n", self.data.to_xml())
    }
}

// #[derive(BinRead, Debug)]
// pub struct ItemStreamInfo {
//     pub a: u64,
//     pub b: u64,
//     pub item_header: ItemHeader,
//     pub c: u8,
// }
//
// #[derive(BinRead, Debug)]
// pub struct ItemHeader {
//     pub a: u64, // size (u64 @ +0x00)
//     pub b: u32, // header flags, deferred flag is header & 1 - default 1
//     pub c: u32, // 'hsin' DomainID (usually 0x4e495344 / NISD, 0x4e494b34 / NIK4)
//     pub d: u32, // default 1
//     pub e: u32, // default 0
//     pub uuid: ItemUuid // checksum?
// }
//
// // uint __cdecl method.NI::SOUND::ItemFrame.read_NI::GP::Stream_(uint *arg_8h, uint arg_ch)
// // stream frame size in bytes = 0x14
#[derive(BinRead, Debug)]
pub struct ItemFrame {
    pub size: u64,
    pub domain_id: u32, // (+8)
    pub item_id: u32,   // (+0xc)
    pub version: u32,   // (+0x10)
}

#[derive(BinRead, Debug)]
pub struct Repository {}

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

#[derive(BinRead, Debug)]
pub struct HeaderChunk {
    pub length: u64,
    pub unknown_a: u32, // item deferred flag?

    #[br(assert(tag==['h','s','i','n']))]
    pub tag: [char; 4],
    pub id: u64,            // uuid?
    pub checksum: [u8; 16], // md5 of child section (including child chunk)
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

    #[br(parse_with = SegmentType::binread)]
    pub id: SegmentType,
    pub chunk: HeaderChunk,
}

impl ChildChunk {
    fn to_xml(&self) -> String {
        self.chunk.to_xml()
    }
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

fn read_data_frames<R: Read + Seek>(
    reader: &mut R,
    ro: &binread::ReadOptions,
    _: (),
) -> BinResult<DataField> {
    let dsin: DataFieldHeader = reader.read_le()?;

    let mut data = vec![0; dsin.length as usize - 20];
    reader.read_exact(&mut data)?;

    let mut data_cursor = Cursor::new(&data);

    let child: Option<Box<DataField>> = match dsin.type_id {
        1 => None,
        _ => Some(Box::new(read_data_frames(&mut data_cursor, ro, ())?)),
    };

    let mut data_raw = Vec::new();
    data_cursor.read_to_end(&mut data_raw)?;
    //let data = NIData::read(dsin.type_id, &data_bin)?;

    Ok(DataField {
        tag: dsin.tag,
        type_id: dsin.type_id.into(),
        unknown_a: dsin.unknown_a,
        data: data_raw,
        child,
    })
}

#[derive(Debug)]
pub struct DataField {
    pub tag: String,
    pub type_id: SegmentType,
    pub unknown_a: u32, // always 1
    pub data: Vec<u8>,
    pub child: Option<Box<DataField>>,
}

struct ByteBuf<'a>(&'a [u8]);

impl<'a> std::fmt::LowerHex for ByteBuf<'a> {
    fn fmt(&self, fmtr: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for byte in self.0 {
            fmtr.write_fmt(format_args!("{:02x}", byte))?;
        }
        Ok(())
    }
}

impl DataField {
    pub fn parse(&self) {
        let mut cursor = Cursor::new(&self.data);

        println!("self.data.len {}", &self.data.len());

        let has_content = cursor.read_u8().unwrap();
        println!("{}", has_content);

        let iVar2: u32 = cursor.read_le().unwrap();
        println!("{}", iVar2);

        match self.type_id {
            SegmentType::RepositoryRoot => {
                // cursor.seek(std::io::SeekFrom::Start(0x2c));
                // let repository_type: i32 = cursor.read_le().unwrap();
                // println!("{}", repository_type);

                cursor.seek(std::io::SeekFrom::Start(0x20 + 0x04)).unwrap();

                let major_version: i32 = cursor.read_le().unwrap();
                println!("-- {:x}", ByteBuf(&major_version.to_le_bytes()));

                // let major_version = major_version >> 0x00;
                println!("--- {:x}", ByteBuf(&major_version.to_le_bytes()));

                // lower 8 bits (al register)
                let major_version = major_version & 0xFFF;
                println!("---- {:x}", ByteBuf(&major_version.to_le_bytes()));

                // let major_version: i32 = cursor.read_le().unwrap();
                // println!("-- {:x}", ByteBuf(&major_version.to_le_bytes()));
                //
                // let major_version = major_version >> 0x14;
                // println!("--- {:x}", ByteBuf(&major_version.to_le_bytes()));
                //
                // // lower 8 bits (al register)
                // let major_version = major_version & 0xFF;
                // println!("---- {:x}", ByteBuf(&major_version.to_le_bytes()));

                // patch version 0x00 3b
                // major version: 0x14 2b
                // minor version 0xc 2b

                cursor.seek(std::io::SeekFrom::Start(0x20 + 0x04)).unwrap();

                let major_version: i32 = cursor.read_le().unwrap();
                println!("-- {:x}", ByteBuf(&major_version.to_le_bytes()));

                let major_version = major_version >> 0xc;
                println!("--- {:x}", ByteBuf(&major_version.to_le_bytes()));
            }
            SegmentType::Authorization => {
                println!("aut");

                println!("{:x}", ByteBuf(&self.data));
                cursor.seek(std::io::SeekFrom::Start(0x14));
                println!("{:?}", cursor);
                let license_info: i32 = cursor.read_be().unwrap();
                println!("{}", license_info);

                // self.child.as_ref().unwrap().parse();
            }
            _ => panic!("{:?}", self.type_id),
        }
    }
}

#[derive(BinRead, Debug)]
pub struct DataFieldHeader {
    pub length: u64,
    #[br(map = |val: [u8; 4]| String::from_utf8_lossy(&val).to_string())]
    pub tag: String,
    // #[br(parse_with = SegmentType::binread)]
    // pub type_id: SegmentType,
    pub type_id: u32,
    pub unknown_a: u32, // always 1
}

impl HeaderChunk {
    pub fn to_xml(&self) -> String {
        format!(
            "<{:?}>{}</{:?}>",
            self.data_chunk.type_id,
            self.children
                .iter()
                .map(ChildChunk::to_xml)
                .collect::<String>(),
            self.data_chunk.type_id
        )
    }
}
