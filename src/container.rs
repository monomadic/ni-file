use rctree::Node;
use crate::Error;
use binread::{io::Cursor, prelude::*};
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
            109 => SegmentType::Preset,
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
    let d: u16 = segment_cursor.read_le()?;
    let e: u16 = segment_cursor.read_le()?;
    warn!("unknown u16 values d, e {:?}", (d, e));

    let child_count: u32 = segment_cursor.read_le()?;
    info!("{} children reported", child_count);

    for _i in 0..child_count {
        let index: u32 = segment_cursor.read_le()?;
        info!("read child index: {} (or is it child count?)", index);

        let magic: String = crate::strings::read_utf8(&mut segment_cursor, 4)?;
        info!("read next magic {:?}", &magic);

        let segment_id: u32 = segment_cursor.read_le()?;
        let segment_type: SegmentType = segment_id.into();

        header(&mut segment_cursor)?;
    }

    if segment_cursor.has_data_left()? {
        error!("header segment has {} unused bytes remaining!", segment_cursor.remaining_slice().len());
    }

    Ok(())
}

fn data_segment(cursor: &mut Cursor<&[u8]>) -> Result<(), Error> {
    info!("reading data segment");

    let size: u64 = cursor.read_le()?;
    info!("reported data segment size: {} bytes", size);

    let mut segment = vec![0; size as usize - 8];
    cursor.read_exact(&mut segment)?;
    let mut segment_cursor: Cursor<&[u8]> = Cursor::new(&segment);

    let magic: String = crate::strings::read_utf8(&mut segment_cursor, 4)?;
    info!("read magic {:?}", &magic);

    let segment_id: u32 = segment_cursor.read_le()?;
    let segment_type: SegmentType = segment_id.into();

    info!("segment id: {} {:?}", segment_id, segment_type);

    match segment_type {
        _ => warn!("skipping segment {}", segment_id),
    }

    if segment_cursor.has_data_left()? {
        error!("data block has {} unused bytes remaining!", segment_cursor.remaining_slice().len());
    }

    Ok(())
}