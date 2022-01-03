use crate::container::SegmentType;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Read;
use std::io::Write;

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
    if b != 1 {
        error!("expected b to be 0, got {}", b);
    }

    let tag: Vec<u8> = read_bytes(&mut current_segment, 4);
    let tag: &str = std::str::from_utf8(&tag).unwrap().into();

    let c = current_segment.read_u32::<LittleEndian>().unwrap();

    if c != 1 {
        error!("expected 1 data segment, got {}", c);
    }

    let d = current_segment.read_u32::<LittleEndian>().unwrap();
    if d != 0 {
        error!("expected d to be 0, got {}", d);
    }

    let checksum = [
        current_segment.read_u32::<LittleEndian>().unwrap(),
        current_segment.read_u32::<LittleEndian>().unwrap(),
        current_segment.read_u32::<LittleEndian>().unwrap(),
        current_segment.read_u32::<LittleEndian>().unwrap(),
    ];

    let _checksum = checksum
        .iter()
        .map(|h| format!("{:08x}", h))
        .collect::<String>();

    info!("<{}>", tag);

    let mut data = data_segment(&current_segment);
    data = pre_hsin(&data);

    if data.len() != 0 {
        error!("data remaining in this hsin segment: {}", data.len());
        dump("dump", data);
    }

    info!("</{}>", tag);

    buffer
}

// 20 byte hsin header
fn pre_hsin(mut buffer: &[u8]) -> &[u8] {
    let a = buffer.read_u16::<LittleEndian>().unwrap();
    let b = buffer.read_u16::<LittleEndian>().unwrap();

    if (a, b) != (1, 0) {
        error!("expected a, b in pre_hsin to be (1,0), got {:?}", (a, b));
    }

    let children = buffer.read_u32::<LittleEndian>().unwrap();
    info!("{} pre_hsin child nodes found", children);

    for _i in 0..children {
        let child_index = buffer.read_u32::<LittleEndian>().unwrap();
        info!("child index {}", child_index);

        // if child_index != i {
        //     error!("incorrect child_index, expected {} got {}", i, child_index);
        // }

        let tag: Vec<u8> = read_bytes(&mut buffer, 4);
        let tag: &str = std::str::from_utf8(&tag).unwrap().into();
        let next_segment_id = buffer.read_u32::<LittleEndian>().unwrap();
        let next_segment_type: SegmentType = next_segment_id.into();

        info!(
            "<pre-{} id={} type='{:?}'>",
            tag, next_segment_id, next_segment_type
        );

        buffer = header_segment(&buffer);
        info!("</pre-{} id={}>", tag, next_segment_id);
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

    let segment_id = current_segment.read_u32::<LittleEndian>().unwrap();
    let segment_type: SegmentType = segment_id.into();

    info!("<{} id={} type='{:?}'>", tag, segment_id, segment_type);

    let data: Vec<u8> = read_bytes(&mut current_segment, (blocksize - 20) as usize);

    let mut file = std::fs::File::create(format!(
        "output/dsin-{}-{:?}.data",
        segment_id, segment_type
    ))
    .unwrap();
    file.write_all(&data).unwrap();

    match segment_type.clone() {
        SegmentType::Version => {
            crate::app_version::read(&data);
        }
        SegmentType::Maybe(s) => {}
        SegmentType::PresetContainer => {
            info!("preset container found {} bytes", data.len());
            read_inner_data_segments(&data);
        }
        SegmentType::PresetInner => {
            let mut data = read_inner_data_segments(&data);

            let mut file = std::fs::File::create("output/preset.compressed").unwrap();
            file.write_all(&data).unwrap();

            // let _ = data.read_u8().unwrap();

            let data = [data, &[0x00, 0x00, 0x00, 0x00]].concat();

            info!(
                "passing {} bytes of compressed data to deflator",
                data.len()
            );

            let (_rem, deflated_data) = crate::deflate::deflate(&data, 11).unwrap();
            let (_, deflated_fixed) = deflated_data.split_at(1);

            let mut file = std::fs::File::create("output/preset.deflated").unwrap();
            file.write_all(&deflated_fixed).unwrap();

            info!("wrote preset to output/preset.deflated");
        }
        SegmentType::CompressedPreset => {
            // info!("found compressed preset {:?}", data);
            // let (_rem, preset_data) = crate::deflate::deflate(&data, 0).unwrap();
            // let mut file = std::fs::File::create("output/preset.deflated").unwrap();
            // file.write_all(&preset_data).unwrap();
        }
        _ => {}
    };

    let c = current_segment.read_u16::<LittleEndian>().unwrap();
    let d = current_segment.read_u16::<LittleEndian>().unwrap();

    if (c, d) != (0, 0) {
        warn!("expected (c, d) in pre_hsin to be (0,0), got {:?}", (c, d));
    }

    info!("</{}>", tag);
    if current_segment.len() != 0 {
        warn!("data remaining in dsin segment: {}", current_segment.len());
    } else {
        // info!("data successfully consumed for data segment");
    }

    buffer
}

// fn read_shortstring(buffer: &mut &[u8]) -> String {
//     let size = buffer.read_u8().unwrap() as usize;
//     let string: Vec<u8> = read_bytes(buffer, size);
//     std::str::from_utf8(&string).unwrap().into()
// }

fn read_bytes(buffer: &mut &[u8], size: usize) -> Vec<u8> {
    let buf: &mut Vec<u8> = &mut vec![0u8; size];
    buffer.read_exact(buf).expect("read_bytes");
    buf.clone()
}

fn read_inner_data_segments(mut buffer: &[u8]) -> &[u8] {
    let count = buffer.read_u32::<LittleEndian>().unwrap();

    info!("data segment with {} children", count);
    let buffer = data_segment(buffer);

    if count != 1 {
        error!("only accounted for 1 child in inner data segments.");
    }

    buffer
}

/// dump bytes to file
fn dump(file: &str, bytes: &[u8]) {
    let mut file = std::fs::File::create(file).unwrap();
    file.write_all(&bytes).unwrap();
}
