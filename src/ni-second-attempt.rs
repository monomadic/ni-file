use bytes::complete::take;
use nom::{
    branch::alt,
    bytes::{self, complete::tag},
    number::complete::{le_i32, le_u32, le_u64},
    sequence::tuple,
    IResult,
};

pub struct NIFile<'a> {
    header: &'a [u8],
    metadata: &'a [u8],
    unknown_segment: &'a [u8],
    compressed_segment: &'a [u8],
    end_segment: &'a [u8],
}

pub fn read(file: &'static [u8]) -> Result<NIFile, Box<dyn std::error::Error>> {
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
    println!(
        "compressed-segment-length found: {}",
        compressed_segment_length
    );

    let (r, _) = take_bytes(r, 8)?;

    let (r, compressed_segment) = take_bytes(r, compressed_segment_length as usize)?;

    let (r, end_segment_length) = take_i32(r)?;
    println!("end_segment_length found: {}", end_segment_length);

    // end segments length tag is inclusive
    let (r, end_segment) = take_bytes(r, (end_segment_length - 4) as usize)?;

    println!("remaining bytes: {}", r.len());

    Ok(NIFile {
        header,
        metadata,
        unknown_segment,
        compressed_segment,
        end_segment,
    })
}

struct NIBlock {
    size: u64,
    unknown_1: u32,
    unknown_2: u64,
    hash: &'static [u8], // [u8; 16]
    section_size: u32,
    data: &'static [u8],
    footer: &'static [u8], // [u8; 20]
}

fn take_block(i: &[u8]) -> IResult<&[u8], NIBlock> {
    let (r, (size, unknown_1, _, unknown_2, hash)) =
        tuple((le_u64, le_u32, tag("hsin"), le_u64, take(16_usize)))(i)?;

    // let (r, size) = take_i32(i)?;
    // println!("file-length found: {}", size);

    Ok((
        r,
        NIBlock {
            size,
            unknown_1,
            unknown_2,
            hash,
            section_size: 0,
            data: &[],
            footer: &[],
        },
    ))
}

fn take_bytes(i: &[u8], l: usize) -> IResult<&[u8], &[u8]> {
    bytes::complete::take(l)(i)
}

fn take_i32(i: &[u8]) -> IResult<&[u8], i32> {
    nom::number::complete::le_i32(i)
}

fn take_u64(i: &[u8]) -> IResult<&[u8], u64> {
    nom::number::complete::le_u64(i)
}
