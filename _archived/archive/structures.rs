use nom::{
    multi::{many1, many_m_n},
    number::complete::{le_u16, le_u32, le_u8},
    sequence::tuple,
    IResult,
};

#[derive(Debug)]
pub struct NIAppVersion {
    pub unknown_1: u32, // always 1?
    pub commercial: u8, // 1 commercial / 0 user
    pub application_id: u32,
    pub unknown_4: u32,
    pub version: String,
}

pub fn parse_app_version(i: &[u8]) -> IResult<&[u8], NIAppVersion> {
    let (rem, (unknown_1, commercial, application_id, unknown_4, string_length)) =
        tuple((le_u32, le_u8, le_u32, le_u32, le_u32))(i)?;
    // println!("string_length {:?}", &string_length);
    let version_data = &rem[0..(string_length * 2) as usize];
    // String::from_utf8(rem[0..string_length as usize].to_vec()).expect("utf 8 header not found");
    // println!("UTF8 {:?}", &version_data);
    let (_, version) = many1(le_u16)(version_data)?;
    // println!("UTF16 {:?}", &version);
    let version = String::from_utf16(&version).expect("utf16 did not convert");

    Ok((
        &[],
        NIAppVersion {
            unknown_1,
            commercial,
            application_id,
            unknown_4,
            version,
        },
    ))
}

// #[derive(Debug)]
// pub struct NICompressedSegment {
//     pub unknown_1: u32, // always 1?
//     pub commercial: u8, // 1 commercial / 0 user
//     pub application_id: u32,
//     pub unknown_4: u32,
//     pub version: String,
// }

#[derive(Debug)]
pub struct NIMetaData {
    pub preset_name: String,
    pub author: String,
    pub company: String,
    pub application: String,
    pub bank_name: String,
    pub tags: Vec<String>,
    pub properties: Vec<String>,
}

// 108
pub fn parse_metadata(i: &[u8]) -> IResult<&[u8], NIMetaData> {
    let (r, (unknown_1, unknown_2, unknown_3, unknown_4)) =
        tuple((le_u32, le_u32, le_u32, le_u32))(i)?;

    let (r, preset_name) = take_utf16(r)?;
    let (r, author) = take_utf16(r)?;
    let (r, company) = take_utf16(r)?;

    // let (_, r) = r.split_at(44);
    let (r, a) = take_utf16(r)?;
    let (r, b) = take_utf16(r)?;

    // FF FF FF FF
    let (_, r) = r.split_at(4);

    // other strings?
    let (_, r) = r.split_at(20);

    let (r, (_, _, has_properties)) = tuple((le_u32, le_u32, le_u32))(r)?;

    // println!("has_properties: {:?}", has_properties);

    let (r, application) = take_utf16(r)?;
    let (r, bank_name) = take_utf16(r)?;

    // array of tags
    let (r, tag_count) = le_u32(r)?;
    let tag_count = tag_count as usize;
    let (r, tags) = many_m_n(tag_count, tag_count, take_utf16)(r)?;
    // println!("tags: {:?}", tags);

    // unknown string
    let (r, unknown_1) = take_utf16(r)?;

    // hashmap of properties
    let (r, properties) = if has_properties == 2 {
        let (r, array_count) = le_u32(r)?;
        let array_count = array_count as usize * 2; // hashmap, so x 2
        many_m_n(array_count, array_count, take_utf16)(r)?
    } else {
        (r, vec![])
    };

    Ok((
        r,
        NIMetaData {
            preset_name,
            author,
            company,
            application,
            bank_name,
            tags,
            properties,
        },
    ))
}

fn take_utf16(i: &[u8]) -> IResult<&[u8], String> {
    let (r, size) = le_u32(i)?;

    if size == 0 {
        return Ok((r, String::new()));
    }

    // println!("size: {}", size);

    let (string_data, r) = r.split_at((size as usize) * 2);

    // println!("d: {:?}", string_data);

    let (_, string_data_wide) = many1(le_u16)(string_data)?;

    let utf16_string = String::from_utf16(&string_data_wide).unwrap(); // fixme unwrap

    Ok((r, utf16_string))
}
