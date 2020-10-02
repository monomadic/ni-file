use crate::strings::{take_utf8, take_utf16};
use nom::{
    bytes::complete::tag,
    number::complete::{le_u32, le_u64, le_f32},
    sequence::tuple,
    IResult,
};

#[derive(Debug)]
pub struct FM8Preset {
    morph_name_bottom_left: String,
    morph_name_bottom_right: String,
    morph_name_top_right: String,
    morph_name_top_left: String,
}

pub fn fm8_preset(i: &[u8]) -> IResult<&[u8], FM8Preset> {
    let (rem, (u1, u2, u3, length, _, u4, u5)) =
        tuple((le_u32, le_u32, le_u32, le_u64, tag("E8MF"), le_u32, le_u32))(i)?;

    // println!("length: {:?}", take_utf8(rem)?);
    
    let (
        rem,
        (
            morph_name_bottom_left,
            morph_name_bottom_right,
            morph_name_top_right,
            morph_name_top_left,
        ),
    ) = tuple((take_utf8, take_utf8, take_utf8, take_utf8))(rem)?;

    println!("val: {:?}", le_f32(rem)?.1);

    Ok((
        &[],
        FM8Preset {
            morph_name_bottom_left,
            morph_name_bottom_right,
            morph_name_top_right,
            morph_name_top_left,
        },
    ))
}
