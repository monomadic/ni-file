use crate::prelude::*;
use binread::{io::Cursor, prelude::*};
use std::io::prelude::*;

pub fn read(data: &[u8]) -> Result<Vec<DataField>> {
    let mut cursor: Cursor<&[u8]> = Cursor::new(data);
    let mut fields: Vec<FieldHeader> = Vec::new();
    let mut data_ends: Vec<u64> = Vec::new();
    let mut data_offset: usize;

    loop {
        let field: FieldHeader = cursor.read_le()?;
        let id = field.id;
        data_ends.push((cursor.position() - 20) + field.size);
        fields.push(field);
        if id == 1_u32 {
            // assert_eq!(cursor.position(), 60);
            data_offset = cursor.position() as usize;
            break;
        }
    }

    // data of last field is always a 1_u32
    // assert_eq!(
    //     cursor.read_le::<u32>()?,
    //     1_u32,
    //     "terminator data is not 1_u32"
    // );

    // where the data starts (not including terminator)
    // this is the size of each header (20 bytes)
    // let data_offset = (fields.len() + 1) * 20;
    // assert_eq!(fields.len(), 3);
    // assert_eq!(data_offset, 60);

    // reverse it because we need to work inside out
    fields.reverse();
    data_ends.reverse();

    let mut data_fields = Vec::new();

    for (index, field) in fields.iter().enumerate() {
        let field_data: Vec<u8> = data.to_vec()[data_offset..data_ends[index] as usize].to_vec();
        data_offset = data_offset + field_data.len();
        data_fields.push(DataField {
            id: field.id,
            data: field_data,
        });
    }

    Ok(data_fields)
}

#[derive(BinRead, Debug)]
pub struct FieldHeader {
    pub size: u64,
    pub tag: u32,
    pub id: u32,
    pub u: u32,
}

#[derive(Debug)]
pub struct DataField {
    pub id: u32,
    pub data: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_field_headers_test() -> Result<()> {
        let file =
            include_bytes!("../data/item-data/118-106-Kontakt5-RepositoryRoot-Authorization.data");

        read(file)?;

        Ok(())
    }
}

pub fn read_data(reader: &mut Cursor<&[u8]>) -> BinResult<NIData> {
    let size: u64 = reader.read_le()?;

    //assert_eq!(reader.bytes().count() + 8, size as usize, "blah"); // TODO: return error

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
