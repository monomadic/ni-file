use std::{fmt::Display, io::Cursor};

use crate::{
    nis::{ItemData, ItemType},
    prelude::*,
    read_bytes::ReadBytesExt,
};

/// Usually the top-level [`Item`][crate::nisound::Item] of a repository. Contains NISound version information.
#[derive(Debug)]
pub struct RepositoryRoot {
    pub nisound_version: RepositoryVersion,
    /// Found values: 0
    pub repository_magic: u32,
    /// Found values: 1, 2
    pub repository_type: u32,
    pub segments: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub struct RepositoryVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl From<u32> for RepositoryVersion {
    fn from(i: u32) -> Self {
        RepositoryVersion {
            major: (i >> 0x14) & 0xff,
            minor: (i >> 0xc) & 0xff,
            patch: i & 0xfff,
        }
    }
}

impl Display for RepositoryVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}.{}.{}", self.major, self.minor, self.patch))
    }
}

impl std::convert::TryFrom<&ItemData> for RepositoryRoot {
    type Error = NIFileError;

    fn try_from(item: &ItemData) -> std::result::Result<Self, Self::Error> {
        if item.header.item_type() != ItemType::RepositoryRoot {
            return Err(NIFileError::ItemWrapError {
                expected: ItemType::RepositoryRoot,
                got: item.header.item_type(),
            });
        }
        RepositoryRoot::read(Cursor::new(item.data.clone()))
    }
}

impl RepositoryRoot {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {
        // itemVersion == 1
        assert_eq!(reader.read_u32_le()?, 1);

        let nisound_version = RepositoryVersion::from(reader.read_u32_le()?);
        let repository_magic = reader.read_u32_le()?;
        let repository_type = reader.read_u32_le()?;

        assert_eq!(reader.read_u32_le()?, 1);

        let num_segments = reader.read_u32_le()?;
        let mut segments = Vec::new();
        for _ in 0..num_segments {
            segments.push(reader.read_widestring_utf16()?);
        }

        // // SNPID::read
        // let a = reader.read_widestring_utf16()?;
        // // assert_eq!(a, "");
        //
        // // UUID::read
        // let a = reader.read_widestring_utf16()?;
        // // assert_eq!(a, "");
        //
        // let a = reader.read_u64_le()?;
        //
        // let mut buf = Vec::new();
        // reader.read_to_end(&mut buf)?;
        //
        // dbg!(&buf, buf.len());

        // repository-type
        // referenced-item-uuid

        Ok(Self {
            nisound_version,
            repository_magic,
            repository_type,
            segments,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn test_repository_root_read_000() -> Result<()> {
        let file =
            File::open("tests/data/Containers/NIS/objects/RepositoryRoot/RepositoryRoot-000")?;
        let item = RepositoryRoot::read(file)?;

        assert_eq!(
            item.nisound_version,
            RepositoryVersion {
                major: 1,
                minor: 7,
                patch: 14,
            }
        );
        assert_eq!(0, item.repository_magic);
        assert_eq!(1, item.repository_type);

        Ok(())
    }

    #[test]
    fn test_repository_root_read_001() -> Result<()> {
        let file =
            File::open("tests/data/Containers/NIS/objects/RepositoryRoot/RepositoryRoot-001")?;
        let item = RepositoryRoot::read(file)?;

        assert_eq!(
            item.nisound_version,
            RepositoryVersion {
                major: 1,
                minor: 4,
                patch: 0,
            }
        );
        assert_eq!(0, item.repository_magic);
        assert_eq!(2, item.repository_type);

        Ok(())
    }
}
