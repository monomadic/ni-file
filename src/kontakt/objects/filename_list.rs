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

use crate::{
    kontakt::{chunk::Chunk, KontaktError},
    read_bytes::ReadBytesExt,
    Error,
};

/// Type:           Chunk
/// SerType:        0x25
/// Versions:       0x50
/// Kontakt 7:      BParameterArraySerBParFX8
/// KontaktIO:      BParamArray<8>
#[derive(Debug)]
pub struct FileNameListPreK51 {
    pub filenames: HashMap<u32, String>,
}

impl FileNameListPreK51 {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let mut filenames = HashMap::new();
        let _ = reader.read_u32_le()?;
        let file_count = reader.read_u32_le()?;

        for i in 0..file_count {
            let mut filename = Vec::new();
            let segments = reader.read_i32_le()?;

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

impl std::convert::TryFrom<&Chunk> for FileNameListPreK51 {
    type Error = Error;

    fn try_from(chunk: &Chunk) -> Result<Self, Self::Error> {
        if chunk.id != 0x3d {
            return Err(KontaktError::IncorrectID {
                expected: 0x3d,
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
    fn test_structured_object() -> Result<(), Error> {
        let file = File::open("tests/data/Objects/KontaktV42/filename_list_pre_k5/4.2.2.4504/000")?;
        FileNameListPreK51::read(file)?;
        Ok(())
    }
}
