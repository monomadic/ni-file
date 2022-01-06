use nom::{number::complete::{le_u16, le_u32}, IResult, multi::many1};
use byteorder::{LittleEndian, ReadBytesExt};
use crate::Error;
use binread::{io::Cursor, BinReaderExt, NullString, NullWideString};

/// read a pascal-style utf16 string
/// https://wiki.lazarus.freepascal.org/Character_and_string_types#WideString
pub(crate) fn take_utf16(i: &[u8]) -> IResult<&[u8], String> {
    let (r, size) = le_u32(i)?;

    if size == 0 {
        return Ok((r, String::new()))
    }

    println!("string size: {}", size);

    let (string_data, r) = r.split_at((size as usize) * 2);

    println!("string data: {:?}", string_data);

    // convert [u8] to [u16]
    let (_, string_data_wide) = many1(le_u16)(string_data)?;

    let utf16_string = String::from_utf16(&string_data_wide).unwrap(); // fixme unwrap

    Ok((r, utf16_string))
}

pub(crate) fn take_utf8(i: &[u8]) -> IResult<&[u8], String> {
    let (r, size) = le_u32(i)?;

    if size == 0 {
        return Ok((r, String::new()))
    }

    let (string_data, r) = r.split_at(size as usize);

    Ok((r, String::from_utf8(string_data.to_owned()).unwrap()))
}

pub fn pascal_string_utf16(mut buf: &[u8]) -> Result<(String, &[u8]), Error> {
    let size = buf.read_u32::<LittleEndian>()?;

    if size == 0 {
        return Ok((String::new(), buf))
    }

    let (string_data, rem) = buf.split_at(((size as usize) + 1_usize) * 2);
    buf = rem;

    let string = Cursor::new(string_data).read_le::<binread::NullWideString>().unwrap().into_string();

    Ok((string, rem))
}
