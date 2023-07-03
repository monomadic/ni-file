use std::io::Read;

use crate::{
    kontakt42::program_data::{ProgramDataV80, ProgramDataVA5},
    read_bytes::ReadBytesExt,
    Error,
};

pub struct StructuredObject;

impl StructuredObject {
    // emulates StucturedObject::doRead
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        match reader.read_u16_le()? {
            0x28 => {
                println!("[0x28]");

                // read the chunk into memory
                let len = reader.read_u32_le()?;
                let reader = reader.read_bytes(len as usize)?;
                let mut reader = reader.as_slice();

                // let reader = read_chunk(&mut reader)?;
                // let mut reader = reader.as_slice();

                println!("readChunked {:?}", reader.read_bool()?);

                // 0x80
                let item_version = reader.read_u16_le()?;
                println!("id 0x{:x}", item_version);

                // PRIVATE DATA
                let length = reader.read_u32_le()?;
                println!("private data length {:?}", length);
                reader.read_bytes(length as usize)?;

                // PROGRAM DATA
                println!("public data length {:?}", reader.read_u32_le()?);
                // K4PL_PubData::create(id, version)
                println!("K4PL_PubData::create(0x{:x}, 0x{:x})", 0x28, item_version);
                match item_version {
                    0x80 => {
                        ProgramDataV80::read(&mut reader)?;
                    }
                    0xA5 => {
                        ProgramDataVA5::read(&mut reader)?;
                    }
                    _ => panic!("ProgramData not supported V{:x}", item_version),
                }

                // CHILDREN DATA
                println!("children_length {:?}", reader.read_u32_le()?);

                // StructuredObject::factory
                let chunk_id = reader.read_u16_le()?;
                match chunk_id {
                    0x3a => {
                        // crate::kontakt42::bparam_array::BParamArray8 {};
                        let array = read_array(&mut reader, 8)?;
                        for item in array.iter() {
                            println!("array-id {:?}", item);
                        }
                    }
                    // 0x53 => {}
                    _ => panic!("StructuredObject::factory(0x{:x})", chunk_id),
                };

                let mut buf = Vec::new();
                reader.read_to_end(&mut buf)?;
                println!("remaining: {} bytes", buf.len());
            }
            _ => panic!("unknown chunk id"),
        }

        Ok(Self {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kontakt_preset_read() -> Result<(), Error> {
        for path in crate::utils::get_files("tests/data/nisound/preset/kontakt/**/*")? {
            println!("\nreading {:?}", path);

            let file = std::fs::File::open(&path)?;

            let chunks = StructuredObject::read(&file)?;

            // top level chunks
            // println!("{:?}", chunks.iter()
            //          .map(|c| format!("0x{:x}-{}", c.0, c.1.len()))
            //          .collect::<Vec<String>>().join(","));
        }

        Ok(())
    }
}
fn read_array<R: ReadBytesExt>(
    mut reader: R,
    items: usize,
) -> Result<Vec<Option<(u16, Vec<u8>)>>, Error> {
    let mut array = Vec::with_capacity(items);

    for _ in 0..items {
        array.push(match reader.read_bool()? {
            true => {
                let item_id = reader.read_u16_le()?;
                let item_length = reader.read_u32_le()?;
                let item_data = reader.read_bytes(item_length as usize)?;

                Some((item_id, item_data))
            }
            false => None,
        })
    }

    println!("array8: {} items {:?}", array.len(), array);

    Ok(array)
}

fn read_chunk<R: ReadBytesExt>(mut reader: R) -> Result<Vec<u8>, Error> {
    let length = reader.read_u32_le()?;
    if length > 0 {
        Ok(reader.read_bytes(length as usize)?)
    } else {
        Ok(Vec::new())
    }
}
