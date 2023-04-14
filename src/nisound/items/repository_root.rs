use crate::{
    nisound::item_frame::{item_id::ItemID, ItemFrame},
    prelude::*,
    read_bytes::ReadBytesExt,
};

/// a data field type representing the topmost level of a repository container.
#[derive(Debug, Clone)]
pub struct RepositoryRoot {
    nisound_version: u32,
    repository_magic: u32,
    repository_type: u32,
}

#[derive(Debug)]
pub struct RepositoryVersion {
    major: u32,
    minor: u32,
    patch: u32,
}

impl std::convert::TryFrom<ItemFrame> for RepositoryRoot {
    type Error = NIFileError;

    fn try_from(frame: ItemFrame) -> std::result::Result<Self, Self::Error> {
        log::debug!("RepositoryRoot::try_from");
        debug_assert_eq!(frame.header.item_id, ItemID::RepositoryRoot);
        RepositoryRoot::read(frame.data.as_slice())
    }
}

impl RepositoryRoot {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {
        log::debug!("RepositoryRoot::read");

        // version == 1
        assert_eq!(reader.read_u32_le()?, 1);

        let nisound_version = reader.read_u32_le()?; // 0x20
        let repository_magic = reader.read_u32_le()?; // 0x24
        let repository_type = reader.read_u32_le()?; // 0x2c

        // FileReference::read  NOTE: NOT EQUAL 1
        // assert_ne!(reader.read_u32_le()?, 1);

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
        crate::utils::setup_logger();

        let file =
            include_bytes!("../../../tests/data/nisound/chunks/item-frame-property/kontakt-5/118-RepositoryRoot.data");

        let root = RepositoryRoot::read(file.as_slice())?;

        assert_eq!(1, root.major_version());
        assert_eq!(7, root.minor_version());
        assert_eq!(14, root.patch_version());
        assert_eq!(0, root.repository_magic);

        Ok(())
    }
}
