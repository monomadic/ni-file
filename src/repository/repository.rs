use crate::prelude::*;
use crate::read_bytes::ReadBytesExt;

use super::{ItemReader, RepositoryRoot};

/// Represents a repository file. Usually has a `RepositoryRoot` as the first enclosing `Item`.
#[derive(Clone)]
pub struct NIRepository(Vec<u8>);

impl NIRepository {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {
        log::debug!("NIRepository::read()");
        Ok(Self(reader.read_sized_data()?))
    }

    pub fn data(&mut self) -> Result<RepositoryRoot> {
        // TODO: this should be TryFrom trait
        let stack = self.frame_stack()?;
        panic!("{:?}", stack);

        // RepositoryRoot::read(self.frame_stack()?.as_slice())
    }
}

impl std::io::Read for NIRepository {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.as_slice().read(buf)
    }
}

impl ItemReader for NIRepository {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_repository_read_test() -> Result<()> {
        crate::utils::setup_logger();

        let mut repo = NIRepository::read(
            include_bytes!("../../tests/data/files/kontakt-7/000-default.nki").as_slice(),
        )?;
        let _root = repo.data()?;

        // TODO: repo props

        Ok(())
    }
}
