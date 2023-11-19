use crate::{
    nis::{ItemContainer, ItemType, RepositoryRoot},
    prelude::*,
};

use super::nis_object::NISObject;

#[derive(Debug)]
pub struct Repository(ItemContainer);

impl Repository {
    pub fn repository_root(&self) -> Option<Result<RepositoryRoot>> {
        self.0
            .find_item::<RepositoryRoot>(&ItemType::RepositoryRoot)
    }

    pub fn children(&self) -> &Vec<ItemContainer> {
        &self.0.children
    }

    pub fn infer_object(&self) -> NISObject {
        if let Some(child) = self.0.children.get(0) {
            child.into()
        } else {
            NISObject::Unknown
        }
    }
}

impl From<ItemContainer> for Repository {
    fn from(ic: ItemContainer) -> Self {
        // TODO: checks
        Self(ic)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_container_read_test() -> Result<()> {
        let file = std::fs::File::open("tests/filetype/NISD/kontakt/7.1.3.0/000-default.nki")?;
        let ic = ItemContainer::read(file)?;
        let repository: Repository = ic.into();
        repository.repository_root().unwrap()?;
        Ok(())
    }
}
