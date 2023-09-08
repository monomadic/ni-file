use std::collections::HashMap;

use crate::{read_bytes::ReadBytesExt, Error};

use super::chunkdata::ChunkData;

#[derive(Debug)]
pub struct FNTableImpl {
    pub special_filetable: HashMap<u32, String>,
    pub sample_filetable: HashMap<u32, String>,
    pub other_filetable: HashMap<u32, String>,
}

#[derive(Debug)]
pub struct FileNameListPreK51 {
    pub filenames: HashMap<u32, String>,
}

pub fn read_filename<R: ReadBytesExt>(mut reader: R) -> Result<Vec<String>, Error> {
    let segments = reader.read_i32_le()?;
    let mut filename = Vec::new();
    for _ in 0..segments {
        filename.push(BFileNameSegment::read(&mut reader)?);
    }
    Ok(filename)
}

impl std::convert::TryFrom<&ChunkData> for FileNameListPreK51 {
    type Error = Error;

    fn try_from(chunk: &ChunkData) -> Result<Self, Self::Error> {
        if chunk.id != 0x3d {
            panic!("fixme: error here");
        }
        let reader = std::io::Cursor::new(&chunk.data);
        Self::read(reader)
    }
}

impl std::convert::TryFrom<&ChunkData> for FNTableImpl {
    type Error = Error;

    fn try_from(chunk: &ChunkData) -> Result<Self, Self::Error> {
        if chunk.id != 0x4b {
            panic!("fixme: error here");
        }
        let reader = std::io::Cursor::new(&chunk.data);
        Self::read(reader)
    }
}

impl FNTableImpl {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let version = reader.read_u16_le()?;
        assert!(version == 2); // hard-coded to 2, throws exception otherwise

        // special filetable
        let file_count = reader.read_u32_le()?; // usually 1, but if higher, extra data included
        let mut special_filetable = HashMap::new();
        for i in 0..file_count {
            special_filetable.insert(i, read_filename(&mut reader)?.join("/"));
        }

        // sample filetable
        let file_count = reader.read_u32_le()?;
        let mut sample_filetable = HashMap::new();
        for i in 0..file_count {
            sample_filetable.insert(i, read_filename(&mut reader)?.join("/"));
        }

        // hashtable?
        for _ in 0..file_count {
            reader.read_u64_le()?;
        }

        // offsets?
        for _ in 0..file_count {
            reader.read_u32_le()?;
        }

        // other filetable
        let file_count = reader.read_u32_le()?;
        let mut other_filetable = HashMap::new();
        for i in 0..file_count {
            other_filetable.insert(i, read_filename(&mut reader)?.join("/"));
        }

        Ok(Self {
            special_filetable,
            sample_filetable,
            other_filetable,
        })
    }
}

impl FileNameListPreK51 {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let _ = reader.read_u32_le()?;
        let file_count = reader.read_u32_le()?;

        let mut filenames = HashMap::new();
        for i in 0..file_count {
            let segments = reader.read_i32_le()?;

            let mut filename = Vec::new();
            for _ in 0..segments {
                let _segment_type = reader.read_i8()?;
                let segment = reader.read_widestring_utf16()?;
                filename.push(segment);
            }

            filenames.insert(i, filename.join("/"));
        }

        Ok(Self { filenames })
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
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<String, Error> {
        let segment_type = reader.read_u8()?;
        Ok(match segment_type {
            1 => {
                // drive
                format!("{}:", reader.read_widestring_utf16()?)
            }
            2 | 3 | 4 | 5 => reader.read_widestring_utf16()?,
            6 => {
                // set special type
                String::new()
            }
            _ => panic!("unknown segment id: {segment_type}"),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn test_structured_object() -> Result<(), Error> {
        let file = File::open("tests/patchdata/KontaktV42/filename_list_pre_k5/4.2.2.4504/000")?;
        FileNameListPreK51::read(file)?;
        Ok(())
    }

    #[test]
    fn test_fntableimpl() -> Result<(), Error> {
        let mut file = File::open("test-data/kontakt/fntableimpl/v02/fntableimplv02-000")?;
        file.read_bytes(6)?; // skip chunk header
        FNTableImpl::read(file)?;

        let mut file = File::open("test-data/kontakt/fntableimpl/v02/fntableimplv02-001")?;
        file.read_bytes(6)?; // skip chunk header
        FNTableImpl::read(file)?;

        let mut file = File::open("test-data/kontakt/fntableimpl/v02/fntableimplv02-002")?;
        file.read_bytes(6)?; // skip chunk header
        FNTableImpl::read(file)?;

        let mut file = File::open("test-data/kontakt/fntableimpl/v02/fntableimplv02-003")?;
        file.read_bytes(6)?; // skip chunk header
        FNTableImpl::read(file)?;

        let mut file = File::open("test-data/Kontakt/FNTableImpl/V02/FNTableImplV02-004")?;
        file.read_bytes(6)?; // skip chunk header
        FNTableImpl::read(file)?;

        Ok(())
    }
}
