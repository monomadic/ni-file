use super::{item::Item, ItemError};
use crate::read_bytes::ReadBytesExt;

/// Represents a repository file. Usually has a `RepositoryRoot` as the first enclosing `Item`.
pub struct Repository(Item);

impl Repository {
    pub fn read<R>(mut reader: R) -> Result<Repository, ItemError>
    where
        R: ReadBytesExt,
    {
        // fn read(&self) -> Result<Repository, NIFileError> {
        let item = Item::read(reader)?;
        Ok(Repository(item))
    }
}
