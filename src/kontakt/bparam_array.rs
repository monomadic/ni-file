use crate::{
    kontakt::{chunkdata::ChunkData, structured_object::StructuredObject},
    read_bytes::ReadBytesExt,
    Error,
};

// id 0x3a
#[doc = include_str!("../../doc/schematics/kontakt/BParamArray.md")]
#[derive(Debug)]
pub struct BParamArray;

impl BParamArray {
    pub fn read<R: ReadBytesExt>(mut reader: R, num_items: u32) -> Result<Self, Error> {
        println!("BParamArray<{num_items}>::read()");

        let unused = reader.read_bool()?;
        assert!(!unused); // always false?

        let version = reader.read_u16_le()?;
        println!("- Version: 0x{version:x}");

        let mut items = Vec::new();

        match version {
            0x11 => {
                panic!("Unsupported BParamArray: v11");
            }
            _ => {
                for _ in 0..num_items {
                    let do_read = reader.read_bool()?;

                    // if doRead != '\0'
                    if do_read {
                        // StructuredObject::factory(id, length)
                        let obj: StructuredObject = ChunkData::read(&mut reader)?.try_into()?;

                        for child in &obj.children {
                            // let child = child.clone();
                            // // println!("{}", crate::utils::format_hex(&child.data));
                            // println!("{obj:?}");
                            // println!("{child:?}");
                            // println!("{:?}", child_data);
                        }

                        items.push(obj);
                    }
                }
            }
        }

        Ok(Self)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use crate::Error;

    use super::*;

    #[test]
    fn test_bparam_array() -> Result<(), Error> {
        let file = File::open("tests/patchdata/KontaktV42/param_array/4.2.2.4504/000")?;
        assert!(BParamArray::read(file, 8).is_ok());
        Ok(())
    }
}
