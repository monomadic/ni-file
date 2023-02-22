use super::{item::Item, ItemError};
use crate::read_bytes::ReadBytesExt;

/// Represents a repository file. Usually has a `RepositoryRoot` as the first enclosing `Item`.
pub struct Repository(Vec<u8>);

impl Repository {
    pub fn read<R>(mut reader: R) -> Result<Repository, ItemError>
    where
        R: ReadBytesExt,
    {
        // fn read(&self) -> Result<Repository, NIFileError> {
        Ok(Repository(reader.read_sized_data()?))
    }
}

impl From<Repository> for Item {
    fn from(r: Repository) -> Self {
        Item(r.0)
    }
}
