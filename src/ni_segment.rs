use crate::Error;
use binread::{io::Cursor, prelude::*};
use std::io::prelude::*;

#[derive(Debug)]
pub enum NIData {
    Unknown(u32),
    Version(Version),
    HeaderInfoA(u32),
    HeaderInfoB(u32),
}

impl NIData {
    pub fn read(id: u32, buffer: &[u8]) -> BinResult<Self> {
        Ok(match id {
            101 => version(buffer)?,
            106 => HeaderInfoA(buffer)?,
            118 => HeaderInfoB(buffer)?,
            _ => NIData::Unknown(id),
        })
    }
}

fn version(buffer: &[u8]) -> BinResult<NIData> {
    println!("segment {} : {}", format_hex(buffer), format_ascii(buffer));

    let mut reader = Cursor::new(buffer);

    let version: Version = reader.read_le()?;

    println!("{:?}", version);

    Ok(NIData::Version(version))
}

#[derive(BinRead, Debug)]
pub struct Version {
    pub unknown_a: u32,
    pub unknown_b: u8,

    #[br(parse_with = read_app)]
    pub app: App,

    pub unknown_c: u32,

    #[br(parse_with = pascal_string_utf16)]
    pub version: String,
}

#[derive(BinRead, Debug, Clone)]
pub enum App {
    GuitarRig,
    Kontakt,
    Reaktor,
    Maschine,
    Massive,
    FM8,
    Unknown(u32),
}

fn read_app<R: Read + Seek>(reader: &mut R, _ro: &binread::ReadOptions, _: (),) -> BinResult<App> {
    let id: u32 = reader.read_le()?;
    Ok(match id {
        1 => App::GuitarRig,
        2 => App::Kontakt,
        4 => App::Reaktor,
        5 => App::Maschine,
        7 => App::Massive,
        8 => App::FM8,
        _ => App::Unknown(id),
    })
}

fn HeaderInfoA(buffer: &[u8]) -> BinResult<NIData> {
    Ok(NIData::HeaderInfoA(118))
}

fn HeaderInfoB(buffer: &[u8]) -> BinResult<NIData> {
    Ok(NIData::HeaderInfoB(106))
}

fn format_hex(buffer: &[u8]) -> String {
    format!("{}", &buffer.iter().map(|x| format!("{:02x} ", x)).collect::<String>())
}

fn format_ascii(buffer: &[u8]) -> String {
    format!("{}", String::from_utf8_lossy(buffer).to_string())
    // format!("{}", &buffer.iter().map(|x| if x.is_ascii() {'s'} else {' '})).collect::<String>())
}

fn pascal_string_utf16<R: Read + Seek>(reader: &mut R, _ro: &binread::ReadOptions, _: (),) -> BinResult<String> {
    let size: u32 = reader.read_le()?;

    info!("string length {}", size);

    if size == 0 {
        return Ok(String::new())
    }

    let string: String = reader.read_le::<binread::NullWideString>()?.into_string();

    Ok(string)
}