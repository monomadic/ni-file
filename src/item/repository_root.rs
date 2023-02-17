use binread::prelude::*;

use crate::{prelude::NIFileError, read_bytes::ReadBytesExt};

/// a data field type representing the topmost level of a repository container.
#[derive(BinRead)]
pub struct RepositoryRoot {}

impl RepositoryRoot {
    pub fn read<R>(reader: R) -> Result<Self, NIFileError>
    where
        R: ReadBytesExt,
    {
        Ok(Self {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::glob_paths;

    #[test]
    fn test_reading_files() -> Result<(), Box<dyn std::error::Error>> {
        crate::utils::setup_logger();

        for path in glob_paths("data/item-frame-data/**/*")? {
            log::info!("reading {:?}", path);
            let file = std::fs::read(&path)?;
            let root = RepositoryRoot::read(file.as_slice())?;
            // TODO: asserts
        }
        Ok(())
    }
}
