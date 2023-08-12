use std::{fmt::Display, io::Cursor};

use crate::{
    nisound::item_frame::{item_id::ItemID, ItemFrame},
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

impl std::convert::TryFrom<&ItemFrame> for RepositoryRoot {
    type Error = NIFileError;

    fn try_from(frame: &ItemFrame) -> std::result::Result<Self, Self::Error> {
        assert_eq!(frame.header.item_id, ItemID::RepositoryRoot);
        RepositoryRoot::read(Cursor::new(frame.data.clone()))
    }
}

impl RepositoryRoot {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {
        log::debug!("RepositoryRoot::read");

        // itemVersion == 1
        assert_eq!(reader.read_u32_le()?, 1);

        let nisound_version = reader.read_u32_le()?; // 0x20
        let repository_magic = reader.read_u32_le()?; // 0x24
        let repository_type = reader.read_u32_le()?; // 0x2c

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
    use super::*;

    #[test]
    fn test_repository_root_read() -> Result<()> {
        let file =
            include_bytes!("../../../tests/data/nisound/chunks/item-frame-property/kontakt-5/118-RepositoryRoot.data");

        let root = RepositoryRoot::read(Cursor::new(file))?;

        assert_eq!(1, root.major_version());
        assert_eq!(7, root.minor_version());
        assert_eq!(14, root.patch_version());
        assert_eq!(0, root.repository_magic);

        Ok(())
    }
}
