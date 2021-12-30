use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Read;

pub(crate) fn read(mut buffer: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    // entire file is a segment, so read it
    let hsin = header_segment(&buffer, buffer.len());

    if hsin.len() != 0 {
        warn!("remaining bytes: {}", hsin.len());
    }

    Ok(())
}

fn header_segment(mut buffer: &[u8], filesize: usize) -> &[u8] {
    // info!("enter hsin at offset {}", filesize - buffer.len());
    // block headers are 40 bytes long.

    // size (2 bytes)
    let blocksize = buffer.read_u64::<LittleEndian>().unwrap();
    // info!("header segment size: {} bytes", blocksize);
    // let mut current_segment: &[u8] = read_slice(&mut buffer, (blocksize - 8) as usize);

    let (mut current_segment, buffer) = buffer.split_at((blocksize - 8) as usize);

    // let a = buffer.read_u32::<LittleEndian>().unwrap();
    let b = current_segment.read_u32::<LittleEndian>().unwrap();
    // warn!("Unknown bytes {:?}", (a, b));

    let tag: Vec<u8> = read_bytes(&mut current_segment, 4);
    let tag: &str = std::str::from_utf8(&tag).unwrap().into();
    // info!("Tag {:?}", tag);

    let c = current_segment.read_u32::<LittleEndian>().unwrap();
    let d = current_segment.read_u32::<LittleEndian>().unwrap();
    // warn!("Unknown bytes {:?}", (c, d));

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

    // let data: Vec<u8> = read_bytes(&mut buffer, blocksize as usize);

    let mut data = data_segment(&current_segment, filesize);

    // warn!("current offset: {}", filesize - data.len());
    // let mut count = 1;
    info!(
        "hsin parsed data with remaining {} bytes. checking for embedded hsin",
        data.len()
    );
    data = pre_hsin(&data);

    // while data.len() != 0 {
    //     // info!("current offset: {}", filesize - data.len());

    //     data = header_segment(&data, filesize);

    //     if data.len() != 0 {
    //         warn!("data remaining in this hsin segment: {}", data.len());
    //         data = pre_hsin(&data);
    //     } else {
    //         info!("data successfully consumed for header segment");
    //     }

    //     info!("back from prehsin {}", data.len());

    //     // info!(
    //     //     "data not consumed. remaining {} bytes. attempting to fetch new segment",
    //     //     data.len()
    //     // );
    // }

    if data.len() != 0 {
        warn!("data remaining in this hsin segment: {}", data.len());
    } else {
        info!("data successfully consumed for hsin segment");
    }

    info!("</{}>", tag);

    buffer
}

// 20 byte hsin header
fn pre_hsin(mut buffer: &[u8]) -> &[u8] {
    let a = buffer.read_u16::<LittleEndian>().unwrap();
    info!("a {}", a);
    let b = buffer.read_u16::<LittleEndian>().unwrap();
    info!("b {}", b);
    let children = buffer.read_u32::<LittleEndian>().unwrap();
    info!("{} child nodes found", children);

    for _ in 0..children {
        let d = buffer.read_u32::<LittleEndian>().unwrap();
        info!("d {}", d);
        let tag: Vec<u8> = read_bytes(&mut buffer, 4);
        let tag: &str = std::str::from_utf8(&tag).unwrap().into();
        let next_segment_type = buffer.read_u32::<LittleEndian>().unwrap();
        info!(
            "<pre-{}#{} unknown={:?}>",
            tag,
            next_segment_type,
            (a, b, d)
        );

        buffer = header_segment(&buffer, 0);
    }

    buffer
}

fn data_segment(mut buffer: &[u8], filesize: usize) -> &[u8] {
    // info!("enter dsin at offset {}", filesize - buffer.len());

    // size (2 bytes)
    let blocksize = buffer.read_u64::<LittleEndian>().unwrap();
    // info!("data segment size: {} bytes", blocksize);
    let (mut current_segment, buffer) = buffer.split_at((blocksize - 8) as usize);

    // let a = buffer.read_u32::<LittleEndian>().unwrap();
    // warn!("Unknown bytes {:?}", (a));

    let tag: Vec<u8> = read_bytes(&mut current_segment, 4);
    let tag: &str = std::str::from_utf8(&tag).unwrap().into();
    // info!("Tag {:?}", tag);

    let segment_type = current_segment.read_u32::<LittleEndian>().unwrap();

    // let c = buffer.read_u32::<LittleEndian>().unwrap();
    // warn!("Unknown bytes {:?}", (b, c));
    info!("<{}#{} size={}>", tag, segment_type, blocksize);

    info!("remaining buffer: {} bytes", current_segment.len());
    let data: Vec<u8> = read_bytes(&mut current_segment, (blocksize - 20) as usize);

    info!("</{}#{}>", tag, segment_type);
    let c = current_segment.read_u32::<LittleEndian>().unwrap();
    // let data = data_segment(current_segment);
    if current_segment.len() != 0 {
        error!(
            "data remaining in dsin segment: {} at offset {}",
            current_segment.len(),
            filesize - current_segment.len()
        );
    } else {
        info!("data successfully consumed for data segment");
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

// fn read_slice<'a>(buffer: &mut &'a [u8], size: usize) -> &'a [u8] {
//     let buf = &mut [0u8; size];
//     buffer.read_exact(buf).expect("read_bytes");
//     buf
// }
