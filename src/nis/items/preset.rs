use crate::{
    nis::{ItemContainer, ItemType, Preset, PresetChunkItemProperties},
    Error,
};

#[derive(Debug)]
pub struct PresetContainer(ItemContainer);

impl PresetContainer {
    pub fn properties(&self) -> Result<Preset, Error> {
        Preset::try_from(&self.0.data)
    }

    /// Attempts to fetch the raw inner preset chunk data
    pub fn preset_data(&self) -> Option<Result<Vec<u8>, Error>> {
        match self.0.find_encryption_item()? {
            Ok(enc) => {
                if enc.is_encrypted {
                    panic!("Item is encrypted.");
                }

                let item = enc.subtree.item().unwrap();

                match item.find_item::<PresetChunkItemProperties>(&ItemType::PresetChunkItem) {
                    Some(preset_chunk_item) => {
                        let chunk = preset_chunk_item.unwrap();
                        let data = chunk.chunk();
                        return Some(Ok(data.to_owned()));
                    }
                    None => todo!(),
                }
            }
            Err(e) => panic!("Error unpacking encryption item: {e}"),
        };
    }
}

impl TryFrom<&ItemContainer> for PresetContainer {
    type Error = Error;

    fn try_from(container: &ItemContainer) -> Result<Self, Self::Error> {
        let id = container.id();
        if id != ItemType::Preset {
            return Err(Error::ItemWrapError {
                expected: ItemType::Preset,
                got: id.clone(),
            });
        }
        Ok(Self(container.clone()))
    }
}
