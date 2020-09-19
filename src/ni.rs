use nom::{
    bytes::complete::{tag, take},
    number::complete::{le_u32, le_u64},
    sequence::tuple,
    IResult,
};

pub fn read(i: &[u8]) -> IResult<&[u8], NISegment> {
    take_block(i)
}
#[derive(Debug)]
pub struct NISegment<'a> {
    length: u64,
    unknown_1: u32,          // always 1?
    checksum: &'a [u8], // [u8; 16]
    data: &'a [u8],
    parents: u32,
    children: u32,
    unknown_2: u32,
    unknown_tag: &'a [u8], // usually DSIN <length> or 4KIN
    content: &'a [u8],
}

fn take_block(i: &[u8]) -> IResult<&[u8], NISegment> {
    let (r, (length, unknown_1, tag, unknown_2, checksum, segment_size)) =
        tuple((le_u64, le_u32, tag("hsin"), le_u64, take(16_usize), le_u64))(i)?;

    println!("segment found. remaining size: {}", length);
    println!("segment size: {}", segment_size);

    let (r, (data, parents, children, unknown_2, unknown_tag)) = tuple((
        take((segment_size - 8) as usize),
        le_u32,
        le_u32,
        le_u32,
        take(16_usize),
    ))(r)?;

    println!("children: {}", children);

    let (r, content) = take(16_usize)(r)?;

    Ok((
        r,
        NISegment {
            length,
            unknown_1,
            checksum: &checksum,
            data: &data,
            parents,
            children,
            unknown_2,
            unknown_tag: &unknown_tag,
            content: &r
        },
    ))
}
