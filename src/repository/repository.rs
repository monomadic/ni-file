use super::item::Item;
use super::{ItemFrame, RepositoryRoot};
use crate::prelude::*;
use crate::read_bytes::ReadBytesExt;

/// Represents a repository file. Usually has a `RepositoryRoot` as the first enclosing `Item`.
/// This is a wrapper around an `Item`.
pub struct Repository(Item);

impl Repository {
    pub fn read<R>(mut reader: R) -> Result<Repository>
    where
        R: ReadBytesExt,
    {
        // fn read(&self) -> Result<Repository, NIFileError> {
        Ok(Repository(Item(reader.read_sized_data()?)))
    }

    pub fn root(&mut self) -> Result<RepositoryRoot> {
        // let item = &self.0;
        let mut frame_stack = self.0.frame_stack()?;
        match frame_stack.frame()? {
            ItemFrame::RepositoryRoot(root) => Ok(root),
            _ => Err(NIFileError::Generic("no frame stack".into())),
        }
    }

    pub fn children(&self) -> Result<Vec<Item>> {
        self.0.children()
    }
}

impl From<Repository> for Item {
    fn from(r: Repository) -> Self {
        r.0
    }
}
