use binread::{io::Cursor, prelude::*};
use std::io::prelude::*;

pub fn read_data(reader: &mut Cursor<&[u8]>) -> BinResult<NIData> {
    let cursor = reader.clone();

    let size: u64 = reader.read_le()?;
    let domain_id: u32 = reader.read_le()?;
    let item_id: u32 = reader.read_le()?;
    let u: u32 = reader.read_le()?;

    let inner_object = match item_id {
        1 => None,
        _ => Some(Box::new(read_data(reader.remaining_slice()))),
    };

    let data = match item_id {
        1 => reader.read_le::<u32>()?,
    }

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

#[derive(BinRead, Debug, Clone)]
pub struct NIData {
    pub size: u64,
    pub domain_id: u32, // (uint, 'hsin')
    pub item_id: u32,   // (uint)
    pub u: u32,
    pub inherits_from: Option<Box<NIData>>,
    pub data: Vec<u8>,
}

fn read_object<R: Read + Seek>(
    reader: &mut R,
    ro: &binread::ReadOptions,
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
