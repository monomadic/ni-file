use rctree::Node;
use crate::Error;
use binread::{io::Cursor, prelude::*, NullWideString};
use std::io::prelude::*;

pub struct NIHeaderSegment {
    data: NIDataSegment,
    children: Node<NIDataSegment>,
}

pub struct NIDataSegment {
    id: u32,
    data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub enum SegmentType {
    FileHeader,
    Version,
    LibraryMetadata,
    Preset,
    PresetContainer,
    PresetInner,
    Maybe(String),
    Unknown(u32),
}

impl From<u32> for SegmentType {
    fn from(id: u32) -> Self {
        match id {
            3 => SegmentType::Maybe("KontaktFile".into()),
            101 => SegmentType::Version,
            108 => SegmentType::LibraryMetadata,
            109 => SegmentType::Preset, // occurs in first header of deflated kontakt
            115 => SegmentType::PresetInner,
            116 => SegmentType::PresetContainer,
            117 => SegmentType::PresetContainer,
            118 => SegmentType::FileHeader,
            121 => SegmentType::Maybe("ContainerPart2".into()),
            _ => SegmentType::Unknown(id),
        }
    }
}

pub fn header(cursor: &mut Cursor<&[u8]>) -> Result<(), Error> {
    // let hsin: HSINHeader = cursor.read_le()?;
    // println!("{:#?}", hsin);
    
    // let mut indent = 0;
    // println!("<{:?}>", hsin.tag);

    _header(cursor)?;

    Ok(())
}

pub fn _header(cursor: &mut Cursor<&[u8]>) -> Result<(), Error> {
    info!("reading header segment");

    let size: u64 = cursor.read_le()?;
    info!("reported segment size: {} bytes", size);

    let mut segment = vec![0; size as usize - 8];
    cursor.read_exact(&mut segment)?;
    let mut segment_cursor: Cursor<&[u8]> = Cursor::new(&segment);

    // continue reading the segment

    let a: u32 = segment_cursor.read_le()?;
    warn!("unknown value a:{} (usually 1)", a);

    let magic: String = crate::strings::read_utf8(&mut segment_cursor, 4)?;
    info!("read magic {:?}", &magic);

    let b: u32 = segment_cursor.read_le()?;
    warn!("unknown value a:{} (usually 1)", b);

    let c: u32 = segment_cursor.read_le()?;
    warn!("unknown value c:{} (usually 0)", c);

    let checksum: [u32;4] = [
        segment_cursor.read_le()?,
        segment_cursor.read_le()?,
        segment_cursor.read_le()?,
        segment_cursor.read_le()?,
    ];

    let checksum = checksum
        .iter()
        .map(|h| format!("{:08x}", h))
        .collect::<String>();
    
    warn!("checksum: {}", checksum);

    // let segment_checksum = crate::checksum::calculate(segment_cursor.remaining_slice());
    // info!("calculated segment checksum: {}", segment_checksum);

    // data segment
    data_segment(&mut segment_cursor)?;

    // child segments
    let d: u32 = segment_cursor.read_le()?;
    warn!("unknown u32 values (index?) d {:?}", (d));

    let child_count: u32 = segment_cursor.read_le()?;
    info!("{} children reported", child_count);



    for _i in 0..child_count {
        let index: u32 = segment_cursor.read_le()?;
        info!("index: {}", index);

        let magic: String = crate::strings::read_utf8(&mut segment_cursor, 4)?;
        info!("read next magic {:?}", &magic);

        let segment_id: u32 = segment_cursor.read_le()?;
        let segment_type: SegmentType = segment_id.into();

        // DEBUG DUMP STUFF
        let mut file = std::fs::File::create(format!("output/{}.hsin", segment_id)).unwrap();
        let mut c = segment_cursor.clone();
        let mut d = segment_cursor.clone();
        let s: u32 = c.read_le()?;
        info!("taking block {} bytes", s);
        let mut buffer = vec![0; s as usize];
        d.read_exact(&mut buffer)?;
        file.write_all(&buffer).unwrap();
        // END DEBUG DUMP

        header(&mut segment_cursor)?;
    }

    if segment_cursor.has_data_left()? {
        error!("header segment has {} unused bytes remaining!", segment_cursor.remaining_slice().len());
    }

    Ok(())
}

fn data_segment(cursor: &mut Cursor<&[u8]>) -> Result<(), Error> {
    info!("reading data segment");
    let mut dsin_pointer = cursor.clone();

    let size: u64 = cursor.read_le()?;
    info!("reported data segment size: {} bytes", size);

    let mut segment = vec![0; size as usize - 8];
    cursor.read_exact(&mut segment)?;
    let mut segment_cursor: Cursor<&[u8]> = Cursor::new(&segment);

    // let segment_copy = segment.clone(); // remove later, used for dumps

    let magic: String = crate::strings::read_utf8(&mut segment_cursor, 4)?;
    info!("read magic {:?}", &magic);

    let segment_id: u32 = segment_cursor.read_le()?;
    let segment_type: SegmentType = segment_id.into();

    info!("segment id: {} {:?}", segment_id, segment_type);

    let unknown: u32 = segment_cursor.read_le()?;

    match segment_type {
        _ => warn!("skipping segment {}", segment_id),
    }

    {
        // DEBUG DUMP STUFF
        let mut file = std::fs::File::create(format!("output/{}.{}.dsin", magic, segment_id)).unwrap();
        let mut buffer = vec![0; size as usize];
        dsin_pointer.read_exact(&mut buffer)?;
        file.write_all(&buffer).unwrap();
        // END DEBUG DUMP
    }


    match segment_id {
        1 => {
            info!("1 - empty segment detected. doing nothing.");
        }
        108 => {
            info!("108: library metadata strings");
            let block: DataSegment = segment_cursor.read_ne().unwrap();
            // let block: Block108 = segment_cursor.read_ne().unwrap();
            info!("108: remaining bytes {:?}", segment_cursor.remaining_slice());
        }
        115 => {
            info!("115: compressed segment?");
            data_segment(&mut segment_cursor);
            // info!("115: remaining bytes {:?}", segment_cursor.remaining_slice());
            let a: u16 = segment_cursor.read_le()?;
            info!("115: a unknown: {:?}", a);

            let compressed_file = segment_cursor.remaining_slice();

            let mut file = std::fs::File::create("output/compressed").unwrap();
            file.write_all(&compressed_file).unwrap();

            let (_, deflated) = crate::deflate::deflate(&compressed_file, 11).unwrap();
            let mut file = std::fs::File::create("output/deflated").unwrap();
            file.write_all(&deflated).unwrap();
        }
        116 => {
            info!("116: preset wrapper? used in kontakt");
            data_segment(&mut segment_cursor);
            info!("116: remaining bytes {:?}", segment_cursor.remaining_slice());
        }
        118 => {
        }
        121 => {
            info!("121: possibly compressed preset");
        },
        _ => (),
    }

    if segment_cursor.has_data_left()? {
        error!("data block has {} unused bytes remaining!", segment_cursor.remaining_slice().len());
    }

    Ok(())
}

// #[derive(BinRead, Debug)]
// struct HSINHeader {
//     size: u64,
//     b: u32,
//     tag: [char;4],

//     #[br(little, count = 16)]
//     checksum: Vec<u8>,

//     c: u32,
//     d: u32,

//     data: DataBlock,

//     e: u32, // always 1
//     children: u64,

//     // inner: Option<Box<HSINHeader>>,

//     // #[br(big, count = size - 88)]
//     // children: Vec<u8>,

//     // next_size: u32,

//     // data: DataBlock,

//     // #[br(if (next.size != 1))]
//     // inner: Option<Box<DSINData>>,

//     // child_type: DataType
// }

// #[derive(BinRead, Debug)]
// struct DataBlock {
//     size: u64,
//     element: DataType,
//     children_count: u32,

//     #[br(count=children_count)]
//     children: Vec<DataBlock>,
// }

#[derive(BinRead, Debug)]
struct DataSegment {
    size: u64,
    tag: [char;4],
    // #[br(magic = b"DSIN", assert(id==108))]
    id: u32,
    unknown: u32,

    #[br(parse_with = binread::until_eof)]
    data: Vec<u8>,
}

#[derive(BinRead, Debug)]
struct Block108 {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    name_size_wide: u32,
    name: NullWideString
}

#[derive(BinRead, Debug)]
struct SegmentHeader {
    size: u64,
    tag: [char;4],
    // #[br(magic = b"DSIN", assert(id==108))]
    id: u32,
    unknown: u32,
}