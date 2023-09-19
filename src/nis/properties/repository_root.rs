use std::{fmt::Display, io::Cursor};

use crate::{
    nis::{ItemData, ItemID},
    prelude::*,
    read_bytes::ReadBytesExt,
};

/// Usually the top-level [`Item`][crate::nisound::Item] of a repository. Contains NISound version information.
#[derive(Debug, Clone)]
pub struct RepositoryRoot {
    pub nisound_version: u32,
    pub repository_magic: u32,
    pub repository_type: u32,
}

#[derive(Debug)]
pub struct RepositoryVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Display for RepositoryVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}.{}.{}", self.major, self.minor, self.patch))
    }
}

impl std::convert::TryFrom<&ItemData> for RepositoryRoot {
    type Error = NIFileError;

    fn try_from(frame: &ItemData) -> std::result::Result<Self, Self::Error> {
        assert_eq!(frame.header.item_id, ItemID::RepositoryRoot);
        RepositoryRoot::read(Cursor::new(frame.data.clone()))
    }
}

impl RepositoryRoot {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {
        // itemVersion == 1
        assert_eq!(reader.read_u32_le()?, 1);

        let nisound_version = reader.read_u32_le()?; // 0x20
        let repository_magic = reader.read_u32_le()?; // 0x24
        let repository_type = reader.read_u32_le()?; // 0x2c

        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        // repositoryReferenceFn
        // FileReference::read NOT EQUAL 1
        // assert_ne!(reader.read_u32_le()?, 1);
        // usually 1
        // panic!("repo ref: {:?}", reader.read_u32_le()?);

        // ItemUuid::read

        Ok(Self {
            nisound_version,
            repository_magic,
            repository_type,
        })
    }

    pub fn major_version(&self) -> u32 {
        (self.nisound_version >> 0x14) & 0xff
    }

    pub fn minor_version(&self) -> u32 {
        (self.nisound_version >> 0xc) & 0xff
    }

    pub fn patch_version(&self) -> u32 {
        self.nisound_version & 0xfff
    }

    pub fn version(&self) -> RepositoryVersion {
        RepositoryVersion {
            major: (self.nisound_version >> 0x14) & 0xff,
            minor: (self.nisound_version >> 0xc) & 0xff,
            patch: self.nisound_version & 0xfff,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn test_repository_root_read() -> Result<()> {
        let file = File::open(
            "tests/data/nisound/chunks/item-frame-property/kontakt-5/118-RepositoryRoot.data",
        )?;
        let repository_root = RepositoryRoot::read(file)?;

        assert_eq!(1, repository_root.major_version());
        assert_eq!(7, repository_root.minor_version());
        assert_eq!(14, repository_root.patch_version());
        assert_eq!(0, repository_root.repository_magic);

        Ok(())
    }
}