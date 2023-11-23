use crate::{
    nis::{ItemContainer, ItemType, PresetChunkItemProperties},
    Error,
};

#[derive(Debug)]
pub struct PresetChunkItem(ItemContainer);

impl PresetChunkItem {
    pub fn children(&self) -> &Vec<ItemContainer> {
        &self.0.children
    }

    pub fn properties(&self) -> Result<PresetChunkItemProperties, Error> {
        self.0
            .find_item::<PresetChunkItemProperties>(&ItemType::PresetChunkItem)
            .ok_or(Error::Static("No PresetChunkItem"))?
    }
}

impl From<ItemContainer> for PresetChunkItem {
    fn from(ic: ItemContainer) -> Self {
        Self(ic)
    }
}

#[cfg(test)]
mod tests {
    use crate::Error;

    use super::*;

    #[test]
    fn ni_container_read_test() -> Result<(), Error> {
        let file = std::fs::File::open("tests/filetype/NISD/kontakt/7.1.3.0/000-default.nki")?;
        let ic = ItemContainer::read(file)?;
        let _item: PresetChunkItem = ic.into();
        Ok(())
    }
}
