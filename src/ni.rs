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
    data: DSINValue<'a>,
    parents: u32,
    unknown_tag: Option<&'a [u8]>, // data segment id
    children: Vec<NISegment<'a>>,
}

#[derive(Debug)]
pub struct NIDataSegment<'a> {
    tag: &'a [u8],
    value_1: u32,
    value_2: u32, // always 1?
    data: &'a [u8],
}

fn take_block(i: &[u8]) -> IResult<&[u8], NISegment> {
    let (rem, (length, unknown_1, tag, unknown_2, checksum, data_segment_size)) =
        tuple((le_u64, le_u32, tag("hsin"), le_u64, take(16_usize), le_u64))(i)?;

    println!("reading entire segment: {} bytes.", length);
    println!("data_segment_size: {}", data_segment_size);

    let (rem, (data, parents, children_count)) = tuple((
        take((data_segment_size - 8) as usize),
        le_u32,
        le_u32,
    ))(rem)?;

    let (_, data) = parse_data_segment(&data)?;

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
            data,
            parents,
            unknown_tag,
            children
        },
    ))
}

#[derive(Debug, Clone)]
pub struct DSINValue<'a> {
    header: String,
    id: u32,
    // unknown_1: u32, // always 1
    data: &'a [u8],
    child: Option<Box<DSINValue<'a>>>,
}

pub fn parse_data_segment<'a>(i: &'a [u8]) -> IResult<&[u8], DSINValue> {
    let (r, (tag, id, unknown_1)) = tuple((take(4_usize), le_u32, le_u32))(i)?;

    let tag = std::str::from_utf8(tag).unwrap_or("error");

    println!("{:?}-", id);

    if id == 1 {
        println!("terminator ID found.");
        let data = r;

        return Ok((r, DSINValue {
            header: tag.into(),
            id, data: data.into(),
            child: None,
        }));
    }

    let (r, next_segment_size) = le_u64(r)?;
    println!("next segment size: {}", next_segment_size);

    let data_length: usize = (r.len() + 8_usize) - next_segment_size as usize;
    let offset = r.len() - data_length;
    println!("{}, {}", offset, next_segment_size);

    let child_data = &r[0..offset];
    let data = &r[offset..r.len()];

    // parse_data_segment(child_data);

    // println!("found data tag: {:?}", (tag, id, unknown_1, &r[offset..r.len()]));

    let (_, child_value) = parse_data_segment(child_data)?;

    use std::{fs::File, io};
    use io::Write;
    let mut buffer = File::create(&format!("dsin-data/{}-{}-{}.dsin", tag, id, data.len())).unwrap();
    buffer.write_all(&data).unwrap();

    Ok((r, DSINValue {
        header: tag.into(),
        id, data,
        child: Some(Box::new(child_value)),
    }))
}
