use super::{
    item_frame::{item_id::ItemID, preset::Preset, ItemFrame},
    Item,
};
use crate::{prelude::*, read_bytes::ReadBytesExt, BNISoundPreset, RepositoryRoot};
use std::convert::TryFrom;

// TODO: in bin this is Container

/// Represents a repository file. Usually has a `RepositoryRoot` as the first enclosing `Item`.
pub struct NIContainer(Item);

impl NIContainer {
    pub fn read<R: ReadBytesExt>(reader: R) -> Result<Self> {
        log::debug!("NIContainer::read()");
        Ok(Self(Item::read(reader)?))
    }

    pub fn root(&self) -> Result<RepositoryRoot> {
        RepositoryRoot::try_from(self.0.data()?)
    }

    pub fn find(&self, item: ItemID) -> Option<ItemFrame> {
        self.0.find(&item)
    }

    pub fn preset(&self) -> Result<Preset> {
        for item in &self.0.children {
            match item.data()?.header.item_id {
                ItemID::BNISoundPreset => {
                    // TODO: cleaner
                    return Ok(BNISoundPreset::try_from(item.data()?)?.preset);
                }
                _ => todo!(),
            }
        }

        todo!()
    }

    pub fn children(&self) -> &Vec<Item> {
        &self.0.children
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_container_read_test() -> Result<()> {
        crate::utils::setup_logger();

        let repo = NIContainer::read(
            include_bytes!("../../tests/data/files/kontakt-7/000-default.nki").as_slice(),
        )?;
        let _root = repo.root()?;

        // TODO: repo props

        Ok(())
    }
}
