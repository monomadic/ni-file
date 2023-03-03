use crate::{
    prelude::NIFileError,
    read_bytes::ReadBytesExt,
    repository::item_frame::{authorization::Authorization, ItemFrameHeader},
};

/// a data field type representing the topmost level of a repository container.
#[derive(Debug, Clone)]
pub struct RepositoryRoot {
    nisound_version: u32,
    repository_magic: u32,
    repository_type: u32,
}

// impl std::io::Read for RepositoryRoot {
//     fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
//         self.0.as_slice().read(buf)
//     }
// }
//
// impl ItemReader for RepositoryRoot {}

#[derive(Debug)]
pub struct RepositoryVersion {
    major: u32,
    minor: u32,
    patch: u32,
}

impl RepositoryRoot {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, NIFileError> {
        log::debug!("Reading RepositoryRoot");

        let header = ItemFrameHeader::read(&mut reader)?;
        log::debug!("ItemFrameHeader: {:?}", &header);

        let buf = reader.read_sized_data()?;
        let _ = Authorization::read(&mut buf.as_slice())?;

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
    fn test_reading_files() -> Result<(), Box<dyn std::error::Error>> {
        crate::utils::setup_logger();

        let path = "tests/data/item-frame/kontakt-4/118-RepositoryRoot.data";
        log::info!("reading {:?}", path);

        let file = std::fs::read(&path)?;
        let root = RepositoryRoot::read(file.as_slice())?;

        assert_eq!(1, root.major_version());
        assert_eq!(7, root.minor_version());
        assert_eq!(14, root.patch_version());
        assert_eq!(0, root.repository_magic);

        Ok(())
    }
}
