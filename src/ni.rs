use nom::{
    bytes::complete::{tag, take},
    number::complete::{le_u32, le_u64},
    sequence::tuple,
    IResult, combinator::opt,
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
    unknown_tag: Option<&'a [u8]>, // usually DSIN <length> or 4KIN
    children: Vec<NISegment<'a>>,
}

fn take_block(i: &[u8]) -> IResult<&[u8], NISegment> {
    let (rem, (length, unknown_1, tag, unknown_2, checksum, segment_size)) =
        tuple((le_u64, le_u32, tag("hsin"), le_u64, take(16_usize), le_u64))(i)?;

    println!("reading entire segment: {} bytes.", length);
    println!("data segment size: {}", segment_size);

    let (rem, (data, parents, children_count)) = tuple((
        take((segment_size - 8) as usize),
        le_u32,
        le_u32,
    ))(rem)?;

    let mut r = rem;

    println!("rem: {}", r.len());

    let unknown_tag = if r.len() == 0 {
        None
    } else {
        let (rem, tag) = take(12_usize)(r)?;
        r = rem;
        Some(tag)
    };


    let mut children = Vec::new();

    println!("looping {} times for children.\n", children_count);
    for _ in 0..children_count {
        let (rem, child) = take_block(r)?;
        r = rem;
        children.push(child);
    }

    Ok((
        r,
        NISegment {
            length,
            unknown_1,
            checksum: &checksum,
            data: &data,
            parents,
            unknown_tag,
            children
        },
    ))
}
