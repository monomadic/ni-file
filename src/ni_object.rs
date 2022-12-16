use binread::{io::Cursor, prelude::*};
use std::io::prelude::*;

pub fn read_data(reader: &mut Cursor<&[u8]>) -> BinResult<NIData> {
    let size: u64 = reader.read_le()?;

    assert_eq!(reader.bytes().count() + 8, size as usize); // TODO: return error

    let domain_id: u32 = reader.read_le()?;
    let item_id: u32 = reader.read_le()?;
    let u: u32 = reader.read_le()?;

    let inner_object = match item_id {
        1 => {
            reader.read_le::<u32>()?;
            None
        }
        _ => Some(Box::new(read_data(reader)?)),
    };

    let mut data = Vec::new();
    reader.read_to_end(&mut data)?;

    // let data = vec![];

    Ok(NIData {
        size,
        domain_id,
        item_id,
        u,
        inherits_from: inner_object,
        data,
    })
}

#[derive(BinRead, Debug, Clone)]
pub struct NIData {
    pub size: u64,
    pub domain_id: u32, // (uint, 'hsin')
    pub item_id: u32,   // (uint)
    pub u: u32,
    pub inherits_from: Option<Box<NIData>>,
    pub data: Vec<u8>,
}

fn _read_object<R: Read + Seek>(
    reader: &mut R,
    _ro: &binread::ReadOptions,
    _: (),
) -> BinResult<NIData> {
    let size: u64 = reader.read_le()?;
    let domain_id: u32 = reader.read_le()?;
    let item_id: u32 = reader.read_le()?;
    let u: u32 = reader.read_le()?;

    let inherits_from = match item_id {
        1 => None,
        _ => None,
        // _ => Some(Box::new(read_object(reader, ro, ())?)),
    };

    // let mut data = Vec::new();
    // reader.read_to_end(&mut data)?;

    let data = vec![];

    Ok(NIData {
        size,
        domain_id,
        item_id,
        u,
        inherits_from,
        data,
    })
}
