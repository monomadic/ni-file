use std::collections::HashMap;

use time::OffsetDateTime;

use crate::{
    kontakt::{chunk::Chunk, objects::BFileName},
    read_bytes::ReadBytesExt,
    Error,
};

/// A table representing external files of different kinds, used in Kontakt 5.1+.
/// Kontakt: FNTableImpl
/// LibKIO: BFileName
#[derive(Debug)]
pub struct FNTableImpl {
    /// List of resources and paths (nkr, search paths)
    pub special_filetable: HashMap<u32, String>,
    /// List of samples (wav, ncw)
    pub sample_filetable: HashMap<u32, String>,
    /// List of sample timestamps
    pub sample_timestamp_table: HashMap<u32, time::Date>,
    /// List of instruments (nki) and internal files (ir samples)
    pub other_filetable: HashMap<u32, String>,
}

impl std::convert::TryFrom<&Chunk> for FNTableImpl {
    type Error = Error;

    fn try_from(chunk: &Chunk) -> Result<Self, Self::Error> {
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
        assert!(version == 2); // hard-coded to 2, kontakt throws exception otherwise

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

        // offsets?
        for _ in 0..file_count {
            let _a = reader.read_u32_le()?;
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
            other_filetable,
            sample_timestamp_table,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn test_fntableimpl() -> Result<(), Error> {
        let mut file = File::open("tests/data/Objects/Kontakt/FNTableImpl/FNTableImpl-000")?;
        file.read_bytes(6)?; // skip chunk header
        FNTableImpl::read(file)?;

        let mut file = File::open("tests/data/Objects/Kontakt/FNTableImpl/FNTableImpl-001")?;
        file.read_bytes(6)?; // skip chunk header
        FNTableImpl::read(file)?;

        let mut file = File::open("tests/data/Objects/Kontakt/FNTableImpl/FNTableImpl-002")?;
        file.read_bytes(6)?; // skip chunk header
        FNTableImpl::read(file)?;

        let mut file = File::open("tests/data/Objects/Kontakt/FNTableImpl/FNTableImpl-003")?;
        file.read_bytes(6)?; // skip chunk header
        FNTableImpl::read(file)?;

        Ok(())
    }

    #[test]
    fn test_fntable_004() -> Result<(), Error> {
        let mut file = File::open("tests/data/Objects/Kontakt/FNTableImpl/FNTableImpl-004")?;
        file.read_bytes(6)?; // skip chunk header
        let _table = FNTableImpl::read(file)?;
        Ok(())
    }

    #[test]
    fn test_fntable_005() -> Result<(), Error> {
        let mut file = File::open("tests/data/Objects/Kontakt/FNTableImpl/FNTableImpl-005")?;
        file.read_bytes(6)?; // skip chunk header
        let _table = FNTableImpl::read(file)?;
        Ok(())
    }

    #[test]
    fn test_fntable_006() -> Result<(), Error> {
        let mut file = File::open("tests/data/Objects/Kontakt/FNTableImpl/FNTableImpl-006")?;
        file.read_bytes(6)?; // skip chunk header
        let _table = FNTableImpl::read(file)?;
        Ok(())
    }
}
