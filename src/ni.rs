use nom::{
    bytes::{self, complete::tag},
    number::complete::le_i32,
    IResult,
};

/// NI FILES
///
/// Schematic:
/// [HEADER]

pub struct NIFile {
    header: NIHeader,
    sections: Vec<NISection>, // todo: needs to be ordered
}

/// 210 bytes
pub struct NIHeader {
    file_length: i32,
    data: Vec<u8>, // 206 bytes
}

pub struct NISection {}

/// parse the ni file format
pub fn parse(file: &[u8]) -> IResult<&[u8], i32> {
    parse_file_header(file)
}

pub fn parse_file_header(file: &[u8]) -> IResult<&[u8], i32> {
    println!("file header");

    // let file_length = &file[0..4];
    let (r, file_length) = le_i32(file)?;

    // check filesize is valid
    if file_length != file.len() as i32 {
        panic!("error: file header does not match up with file size.\n");
    } else {
        println!("file_length: {} bytes.", file_length);
    }

    let (r, file_length) = le_i32(r)?;
    let (r, file_length) = le_i32(r)?;

    let (r, header_section_tag) = tag("hsin")(r)?;

    let (r, remaining_bytes) = bytes::complete::take(194 as usize)(r)?;

    let (r, remaining_file_length) = le_i32(r)?;

    if remaining_file_length - 4 != r.len() as i32 {
        panic!(
            "error: remaining_file_length does not match up with remaining file size. {}, {}\n",
            remaining_file_length - 4,
            r.len()
        );
    } else {
        println!("remaining_file_length: {} bytes.\n", remaining_file_length);
    }

    Ok((r, file_length))
}
