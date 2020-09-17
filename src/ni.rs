use nom::{
    bytes::{self, complete::tag},
    number::complete::le_i32,
    IResult,
};

pub fn read(file: &'static [u8]) -> Result<(), Box<dyn std::error::Error>> {
    let (r, file_length) = take_i32(file)?;
    println!("file-length found: {}", file_length);

    let (r, header) = take_bytes(r, 393)?;

    let (r, metadata_length) = take_i32(r)?;
    println!("metadata-length found: {}", metadata_length);

    // skip 8 bytes
    let (r, _) = take_bytes(r, 8)?;

    let (r, metadata) = take_bytes(r, metadata_length as usize)?;

    let (r, unknown_segment_length) = take_i32(r)?;
    println!("unknown-segment-length found: {}", unknown_segment_length);

    // skip 8 bytes
    let (r, _) = take_bytes(r, 8)?;

    // read unknown_segment
    let (r, unknown_segment) = take_bytes(r, unknown_segment_length as usize)?;

    let (r, compressed_segment_length) = take_i32(r)?;
    println!("compressed-segment-length found: {}", compressed_segment_length);

    let (r, _) = take_bytes(r, 8)?;

    let (r, compressed_segment) = take_bytes(r, compressed_segment_length as usize)?;

    let (r, end_segment_length) = take_i32(r)?;
    println!("end_segment_length found: {}", end_segment_length);

    // end segments length tag is inclusive
    let (r, end_segment) = take_bytes(r, (end_segment_length - 4) as usize)?;

    println!("remaining bytes: {}", r.len());

    Ok(())
}

fn take_bytes(i: &[u8], l: usize) -> IResult<&[u8], &[u8]> {
    bytes::complete::take(l)(i)
}

fn take_i32(i: &[u8]) -> IResult<&[u8], i32> {
    nom::number::complete::le_i32(i)
}