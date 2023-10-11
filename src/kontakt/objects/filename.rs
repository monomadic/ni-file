use crate::{read_bytes::ReadBytesExt, Error};

pub struct BFileName;
pub struct BFileNameSegment;

impl BFileName {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Vec<String>, Error> {
        let segments = reader.read_i32_le()?;
        let mut filename = Vec::new();
        for _ in 0..segments {
            filename.push(BFileNameSegment::read(&mut reader)?);
        }
        Ok(filename)
    }

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

/// Internally, kontakt breaks paths into segments for multiplatform support.
impl BFileNameSegment {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<String, Error> {
        let segment_type = reader.read_u8()?;
        Ok(match segment_type {
            1 => {
                // root file node
                format!("{}:", reader.read_widestring_utf16()?)
            }
            3 => {
                // parent dir
                String::from("..")
            }
            2 | 4 | 5 => reader.read_widestring_utf16()?,
            6 => {
                // set special location
                String::new()
            }
            9 => {
                // multi file (used like a dir)
                reader.read_widestring_utf16()?
            }
            _ => panic!("unknown segment id: {segment_type}"),
        })
    }
}
