use nom::{
    number::complete::{le_u32, le_u8, le_u16},
    sequence::tuple,
    IResult, multi::many1,
};

#[derive(Debug)]
pub struct NIAppVersion {
    pub unknown_1: u32,
    pub unknown_2: u8,
    pub unknown_3: u32,
    pub unknown_4: u32,
    pub version: String,
}

pub fn parse_app_version(i: &[u8]) -> IResult<&[u8], NIAppVersion> {
    let (rem, (unknown_1, unknown_2, unknown_3, unknown_4, string_length)) = tuple((le_u32, le_u8, le_u32, le_u32, le_u32))(i)?;
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
            unknown_2,
            unknown_3,
            unknown_4,
            version,
        },
    ))
}
