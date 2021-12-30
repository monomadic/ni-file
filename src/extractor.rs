use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Read;

pub(crate) fn read(mut buffer: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    // entire file is a segment, so read it
    let hsin = header_segment(&buffer);

    let hsin = pre_hsin(&hsin);
    let hsin = header_segment(&hsin);

    let hsin = pre_hsin(&hsin);
    let hsin = header_segment(&hsin);

    let hsin = pre_hsin(&hsin);
    let hsin = header_segment(&hsin);

    let hsin = pre_hsin(&hsin);
    let hsin = header_segment(&hsin);

    // let hsin = pre_hsin(&hsin);
    // let hsin = header_segment(&hsin);

    if hsin.len() != 0 {
        warn!("remaining bytes: {}", hsin.len());
    }

    Ok(())
}

fn header_segment(mut buffer: &[u8]) -> &[u8] {
    // block headers are 40 bytes long.

    // size (2 bytes)
    let blocksize = buffer.read_u64::<LittleEndian>().unwrap();

    if blocksize != buffer.len() as u64 {
        warn!(
            "incorrect hsin blocksize: buffer remaining {} and size is {}",
            buffer.len(),
            blocksize
        );
    }
    // info!("File size: {} bytes", blocksize);

    // let a = buffer.read_u32::<LittleEndian>().unwrap();
    let b = buffer.read_u32::<LittleEndian>().unwrap();
    // warn!("Unknown bytes {:?}", (a, b));

    let tag: Vec<u8> = read_bytes(&mut buffer, 4);
    let tag: &str = std::str::from_utf8(&tag).unwrap().into();
    // info!("Tag {:?}", tag);

    let c = buffer.read_u32::<LittleEndian>().unwrap();
    let d = buffer.read_u32::<LittleEndian>().unwrap();
    // warn!("Unknown bytes {:?}", (c, d));

    let checksum = [
        buffer.read_u32::<LittleEndian>().unwrap(),
        buffer.read_u32::<LittleEndian>().unwrap(),
        buffer.read_u32::<LittleEndian>().unwrap(),
        buffer.read_u32::<LittleEndian>().unwrap(),
    ];

    let checksum = checksum
        .iter()
        .map(|h| format!("{:08x}", h))
        .collect::<String>();

    info!(
        "[{}] size={} checksum={} unknown={:?}",
        tag,
        blocksize,
        checksum,
        (b, c, d)
    );

    // let data: Vec<u8> = read_bytes(&mut buffer, blocksize as usize);

    let buffer = data_segment(&buffer);

    buffer
}

fn data_segment(mut buffer: &[u8]) -> &[u8] {
    // size (2 bytes)
    let blocksize = buffer.read_u64::<LittleEndian>().unwrap();
    // info!("Segment size: {} bytes", blocksize);

    // let a = buffer.read_u32::<LittleEndian>().unwrap();
    // warn!("Unknown bytes {:?}", (a));

    let tag: Vec<u8> = read_bytes(&mut buffer, 4);
    let tag: &str = std::str::from_utf8(&tag).unwrap().into();
    // info!("Tag {:?}", tag);

    let segment_type = buffer.read_u32::<LittleEndian>().unwrap();

    // let c = buffer.read_u32::<LittleEndian>().unwrap();
    // warn!("Unknown bytes {:?}", (b, c));
    info!(" [{}:{}] size={}", tag, segment_type, blocksize,);

    let data: Vec<u8> = read_bytes(&mut buffer, (blocksize - 20) as usize);

    // if segment_type == 4 {
    //     let data: Vec<u8> = read_bytes(&mut buffer, (blocksize - 20) as usize);
    // } else {
    //     let data: Vec<u8> = read_bytes(&mut buffer, blocksize as usize);
    // }

    buffer
}

fn pre_hsin(mut buffer: &[u8]) -> &[u8] {
    let a = buffer.read_u16::<LittleEndian>().unwrap();
    let aa = buffer.read_u16::<LittleEndian>().unwrap();
    let b = buffer.read_u32::<LittleEndian>().unwrap();
    let c = buffer.read_u32::<LittleEndian>().unwrap();
    let d = buffer.read_u32::<LittleEndian>().unwrap();

    let tag: Vec<u8> = read_bytes(&mut buffer, 4);
    let tag: &str = std::str::from_utf8(&tag).unwrap().into();
    let next_segment_type = buffer.read_u32::<LittleEndian>().unwrap();

    info!(
        "--{}: tag={} unknown={:?}",
        next_segment_type,
        tag,
        (a, aa, b, c, d)
    );

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
