use crate::container::SegmentType;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Read;

pub(crate) fn read(buffer: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    // entire file is a segment, so read it
    let hsin = header_segment(&buffer);

    if hsin.len() != 0 {
        warn!("remaining bytes: {}", hsin.len());
    }

    Ok(())
}

fn header_segment(mut buffer: &[u8]) -> &[u8] {
    // size (2 bytes)
    let blocksize = buffer.read_u64::<LittleEndian>().unwrap();

    let (mut current_segment, buffer) = buffer.split_at((blocksize - 8) as usize);

    let b = current_segment.read_u32::<LittleEndian>().unwrap();

    let tag: Vec<u8> = read_bytes(&mut current_segment, 4);
    let tag: &str = std::str::from_utf8(&tag).unwrap().into();

    let c = current_segment.read_u32::<LittleEndian>().unwrap();
    let d = current_segment.read_u32::<LittleEndian>().unwrap();

    let checksum = [
        current_segment.read_u32::<LittleEndian>().unwrap(),
        current_segment.read_u32::<LittleEndian>().unwrap(),
        current_segment.read_u32::<LittleEndian>().unwrap(),
        current_segment.read_u32::<LittleEndian>().unwrap(),
    ];

    let checksum = checksum
        .iter()
        .map(|h| format!("{:08x}", h))
        .collect::<String>();

    info!(
        "<{} size={} checksum={} unknown={:?}>",
        tag,
        blocksize,
        checksum,
        (b, c, d)
    );

    let mut data = data_segment(&current_segment);

    // info!(
    //     "hsin parsed data with remaining {} bytes. checking for embedded hsin",
    //     data.len()
    // );
    data = pre_hsin(&data);

    if data.len() != 0 {
        warn!("data remaining in this hsin segment: {}", data.len());
    } else {
        // info!("data successfully consumed for hsin segment");
    }

    info!("</{}>", tag);

    buffer
}

// 20 byte hsin header
fn pre_hsin(mut buffer: &[u8]) -> &[u8] {
    let a = buffer.read_u16::<LittleEndian>().unwrap();
    let b = buffer.read_u16::<LittleEndian>().unwrap();
    let children = buffer.read_u32::<LittleEndian>().unwrap();
    // info!("{} child nodes found", children);

    for _ in 0..children {
        let d = buffer.read_u32::<LittleEndian>().unwrap();

        let tag: Vec<u8> = read_bytes(&mut buffer, 4);
        let tag: &str = std::str::from_utf8(&tag).unwrap().into();
        let next_segment_type: SegmentType = buffer.read_u32::<LittleEndian>().unwrap().into();

        info!(
            "<pre-{}#{:?} unknown={:?}>",
            tag,
            next_segment_type,
            (a, b, d)
        );

        buffer = header_segment(&buffer);
        info!("</pre-{}#{:?}>", tag, next_segment_type);
    }

    buffer
}

fn data_segment(mut buffer: &[u8]) -> &[u8] {
    // info!("enter dsin at offset {}", filesize - buffer.len());

    // size (2 bytes)
    let blocksize = buffer.read_u64::<LittleEndian>().unwrap();

    let (mut current_segment, buffer) = buffer.split_at((blocksize - 8) as usize);

    let tag: Vec<u8> = read_bytes(&mut current_segment, 4);
    let tag: &str = std::str::from_utf8(&tag).unwrap().into();

    let segment_type: SegmentType = current_segment.read_u32::<LittleEndian>().unwrap().into();

    info!("<{}#{:?} size={}>", tag, segment_type, blocksize);

    let data: Vec<u8> = read_bytes(&mut current_segment, (blocksize - 20) as usize);
    match segment_type.clone() {
        SegmentType::Version => {
            crate::app_version::read(&data);
        }
        SegmentType::Maybe(s) => {
            use std::io::Write;
            let mut buffer = std::fs::File::create(format!("output/{}.data", &s)).unwrap();
            buffer.write_all(&data).unwrap();
        }
        _ => (),
    };

    let c = current_segment.read_u16::<LittleEndian>().unwrap();
    let d = current_segment.read_u16::<LittleEndian>().unwrap();
    info!("</{}#{:?} unknown={:?}>", tag, segment_type, (c, d));
    if current_segment.len() != 0 {
        error!("data remaining in dsin segment: {}", current_segment.len());
    } else {
        // info!("data successfully consumed for data segment");
    }

    buffer
}

fn read_shortstring(buffer: &mut &[u8]) -> String {
    let size = buffer.read_u8().unwrap() as usize;
    let string: Vec<u8> = read_bytes(buffer, size);
    std::str::from_utf8(&string).unwrap().into()
}

fn read_bytes(buffer: &mut &[u8], size: usize) -> Vec<u8> {
    let buf: &mut Vec<u8> = &mut vec![0u8; size];
    buffer.read_exact(buf).expect("read_bytes");
    buf.clone()
}
