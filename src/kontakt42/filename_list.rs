use crate::{read_bytes::ReadBytesExt, Error};

pub struct FileNameListPreK51;

impl FileNameListPreK51 {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        println!("FileNameListPreK51::read");

        println!("_ {}", reader.read_u32_le()?);

        let file_count = reader.read_u32_le()?;
        println!("file_count {}", file_count);

        for _ in 0..file_count {
            let segments = reader.read_i32_le()?;
            println!("segments {}", segments);

            for _ in 0..segments {
                let segment_type = reader.read_i8()?;
                println!("segment_type {}", segment_type);

                let segment = reader.read_widestring_utf16()?;
                println!("{}", segment);
            }
        }

        Ok(Self {})
    }
}

struct BFileName;

impl BFileName {
    // K4PatchLib::BFileName::Retrieve
    pub fn read_filename<R: ReadBytesExt>(mut reader: R) -> Result<BFileName, Error> {
        let i = reader.read_i32_le()?;
        if i < 0 {
            reader.read_widestring_utf16()?;
        } else if i > 0 {
        }

        Ok(BFileName)
    }
}

struct BFileNameSegment;

impl BFileNameSegment {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<BFileNameSegment, Error> {
        let i = reader.read_i8()?;
        // if i < 11
        if i < 0xb {
            let a = reader.read_u16_le()?;
            // if (0x316U >> (uVar5 & 0x1F)) & 1 == 0 {}
            if false {
                match i {
                    5 => {}
                    10 => {}
                    _ => panic!(),
                }
            } else {
                let s = reader.read_widestring_utf16()?;
            }
        }
        if i < 0 {
            reader.read_widestring_utf16()?;
        } else if i > 0 {
        }

        Ok(BFileNameSegment)
    }
}

#[test]
fn test_structured_object() -> Result<(), Error> {
    let file = include_bytes!("tests/filename_list_pre_k5/4.2.2.4504/000");
    let mut file = file.as_slice();
    FileNameListPreK51::read(&mut file)?;

    Ok(())
}
