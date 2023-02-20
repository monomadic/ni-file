use crate::{prelude::NIFileError, read_bytes::ReadBytesExt};

// Authorization
// u32 Value repository-magic 0x24
// u32 Value repository-version 0x28
// u32 Value repository-type 0x2c

// method.NI::SOUND::RepositoryRoot.readItem_NI::GP::Stream__NI::SOUND::ReadContext_

// "DSIN"
// 118
//
// Authorization
//
// u32 0x1
// u32 Lib.getCompiledVersion() 0x107004
// u32 +0x24
// u32 +0x2c
// FileReference +0x40
// Uuid
//

// getNISoundMajorVersion { return *(arg_8h + 0x20) >> 0x14 & 0xff; }
// getNISoundMinorVersion
// getNISoundPatchVersion
// getNISoundVersion +0x20 (32)
// getRepositoryMagic +0x24 (36)
// getRepositoryVersion +0x28
// getRepositoryType +0x2c (44)
// getRepositoryReferenceFn
// getRepositoryItemUuid
// getDomainID
// getItemID { return "DSIN" }

/// a data field type representing the topmost level of a repository container.
pub struct RepositoryRoot {
    version: u32,
    magic: u32,
}

impl RepositoryRoot {
    pub fn read<R>(mut reader: R) -> Result<Self, NIFileError>
    where
        R: ReadBytesExt,
    {
        let _ = reader.read_bytes(0x4)?; // skip initial 4 bytes
        let _ = reader.read_bytes(0x20)?; // skip 20 bytes

        Ok(Self {
            version: reader.read_u32_le()?,
            magic: reader.read_u32_le()?,
        })
    }

    pub fn major_version(&self) -> u32 {
        (self.version >> 0x14) & 0xff
    }

    pub fn minor_version(&self) -> u32 {
        (self.version >> 0xc) & 0xff
    }

    pub fn patch_version(&self) -> u32 {
        self.version & 0xfff
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading_files() -> Result<(), Box<dyn std::error::Error>> {
        crate::utils::setup_logger();

        let path = "tests/data/item-frame-data-field/kontakt-5/118-RepositoryRoot.data";
        log::info!("reading {:?}", path);

        let file = std::fs::read(&path)?;
        let root = RepositoryRoot::read(file.as_slice())?;

        assert_eq!(3, root.major_version());
        assert_eq!(0, root.minor_version());
        assert_eq!(48, root.patch_version());

        assert_eq!(48, root.magic);

        Ok(())
    }
}
