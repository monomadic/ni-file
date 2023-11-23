// FNTableImpl
//
// RetrieveFNTableImpl():
//
// read u16: version
// if version < 2
//   throw BWrongValue
// if version != 2
//   throw BFutureVersion
//
// read u32: numTableItems?
//
// if (some conditions met)
//   ReadFN() // also K4PatchLib::BFileName::Retrieve()
//
// if (some conditions met)
//   ReadFN()
//
// if (some conditions met)
//   read u64 (ctime?)
// if (cond)
//   read u32
//
// read u32
//
// ReadFN while ..
//
// if ..
//   read i16 while ..
//
//
// StoreFNTableImpl():
//
// write u16 = 2
// write u32

use std::{collections::HashMap, io::Cursor};

use time::OffsetDateTime;

use crate::{
    kontakt::{chunk::Chunk, KontaktError},
    read_bytes::ReadBytesExt,
    Error,
};

use super::BFileName;

const CHUNK_ID: u16 = 0x3D;

/// Type:           Chunk<Raw>
/// SerType:        0x3D
/// Kontakt 7:      FNTablePreK51
/// KontaktIO:      FileNameListPreK51
#[derive(Debug)]
pub struct FileNameListPreK51 {
    /// List of resources and paths (nkr, search paths)
    pub special_filetable: HashMap<u32, String>,
    /// List of samples (wav, ncw)
    pub sample_filetable: HashMap<u32, String>,
    /// List of sample timestamps
    pub sample_timestamp_table: HashMap<u32, time::Date>,
    /// List of instruments (nki) and internal files (ir samples)
    pub other_filetable: HashMap<u32, String>,
}

impl FileNameListPreK51 {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        // special filetable
        let file_count = reader.read_u32_le()?;
        let mut special_filetable = HashMap::new();
        for i in 0..file_count {
            special_filetable.insert(i, BFileName::read(&mut reader)?.join("/"));
        }

        // sample filetable
        let file_count = reader.read_u32_le()?;
        let mut sample_filetable = HashMap::new();
        for i in 0..file_count {
            sample_filetable.insert(i, BFileName::read(&mut reader)?.join("/"));
        }

        // sample timestamps
        let mut sample_timestamp_table = HashMap::new();
        for i in 0..file_count {
            let unix_timestamp = reader.read_u64_le()? as i64;
            let datetime = OffsetDateTime::from_unix_timestamp(unix_timestamp).unwrap();
            let timestamp: time::Date = datetime.date();
            sample_timestamp_table.insert(i, timestamp);
        }

        // other filetable
        let file_count = reader.read_u32_le()?;
        let mut other_filetable = HashMap::new();
        for i in 0..file_count {
            other_filetable.insert(i, BFileName::read(&mut reader)?.join("/"));
        }

        Ok(Self {
            special_filetable,
            sample_filetable,
            sample_timestamp_table,
            other_filetable,
        })
    }
}

impl std::convert::TryFrom<&Chunk> for FileNameListPreK51 {
    type Error = Error;

    fn try_from(chunk: &Chunk) -> Result<Self, Self::Error> {
        if chunk.id != CHUNK_ID {
            return Err(KontaktError::IncorrectID {
                expected: CHUNK_ID,
                got: chunk.id,
            }
            .into());
        }
        let reader = Cursor::new(&chunk.data);
        Self::read(reader)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn test_fntable_prek51_003() -> Result<(), Error> {
        let file =
            File::open("tests/data/Objects/Kontakt/0x3D-FNTablePreK51/FNTablePreK51-003.kon")?;
        let chunk = Chunk::read(file)?.data;
        FileNameListPreK51::read(&mut Cursor::new(chunk))?;
        Ok(())
    }
}
