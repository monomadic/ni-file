use super::{item::Item, item_frame::repository_root::RepositoryRoot, ItemError};
use crate::read_bytes::ReadBytesExt;

/// Represents a repository file. Usually has a `RepositoryRoot` as the first enclosing `Item`.
pub struct Repository(pub Item);

impl Repository {
    pub fn read<R>(reader: R) -> Result<Repository, ItemError>
    where
        R: ReadBytesExt,
    {
        // fn read(&self) -> Result<Repository, NIFileError> {
        let item = Item::read(reader)?;
        Ok(Repository(item))
    }

    pub fn root_item(&self) -> Result<RepositoryRoot, ItemError> {
        Ok(RepositoryRoot::read((self.0).0.as_slice()).expect("fix this"))
    }
}
