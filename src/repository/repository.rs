use super::Item;
use crate::{prelude::*, read_bytes::ReadBytesExt, RepositoryRoot};
use std::convert::TryInto;

// TODO: in bin this is Container

/// Represents a repository file. Usually has a `RepositoryRoot` as the first enclosing `Item`.
pub struct NIRepository(Item);

impl NIRepository {
    pub fn read<R: ReadBytesExt>(reader: R) -> Result<Self> {
        log::debug!("NIRepository::read()");
        Ok(Self(Item::read(reader)?))
    }

    pub fn root(&self) -> Result<RepositoryRoot> {
        self.0.frame()?.try_into()
    }

    // pub fn preset(&self) -> Result<BNISoundPreset> {
    //     for item in &self.0.children {
    //         match item.frame()?.header.item_id {
    //             ItemID::BNISoundPreset => (),
    //             _ => (),
    //         }
    //     }
    //
    //     todo!()
    // }

    pub fn children(&self) -> &Vec<Item> {
        &self.0.children
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_repository_read_test() -> Result<()> {
        crate::utils::setup_logger();

        let repo = NIRepository::read(
            include_bytes!("../../tests/data/files/kontakt-7/000-default.nki").as_slice(),
        )?;
        let _root = repo.root()?;

        // TODO: repo props

        Ok(())
    }
}
