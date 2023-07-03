use crate::{
    kontakt42::{
        program_data::{ProgramDataV80, ProgramDataVA5},
        zone_list::ZoneList,
    },
    read_bytes::ReadBytesExt,
    Error,
};

use super::filename_list::FileNameListPreK51;

pub struct StructuredObject;

impl StructuredObject {
    // emulates StucturedObject::doRead
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let id = reader.read_u16_le()?;
        match id {
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
                let children_data_length = reader.read_u32_le()? as usize;
                println!("children_length {:?}", children_data_length);

                // read all children into memory
                let children_data = reader.read_bytes(children_data_length)?;
                let mut reader = children_data.as_slice();

                // StructuredObject::factory
                while let Ok(chunk_id) = reader.read_u16_le() {
                    println!("child id: 0x{:x}", chunk_id);
                    let item_length = reader.read_u32_le()?;
                    let item_data = reader.read_bytes(item_length as usize)?;

                    match chunk_id {
                        0x3a => {
                            // BParamArray<8>
                        }
                        0x32 => {
                            // VoiceGroups
                        }
                        0x33 => {
                            // GroupList
                        }
                        0x34 => {
                            // ZoneList
                            ZoneList::read(&mut item_data.as_slice())?;
                        }
                        // 0x3e => {
                        //     // StructuredObject::doRead(0x3e)
                        //     // readChunked
                        //     assert!(reader.read_bool()?);
                        //     StructuredObject::read(&mut reader)?;
                        // }
                        // 0x53 => {}
                        _ => panic!("StructuredObject::factory(0x{:x})", chunk_id),
                    }
                }
            }
            0x3d => {
                // FileNameListPreK51
                FileNameListPreK51::read(&mut reader)?;
            }
            _ => panic!("Unknown StructuredObject 0x{:x}", id),
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

            let _chunks = StructuredObject::read(&file)?;
            let _chunks = StructuredObject::read(&file)?;
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
