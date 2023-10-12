use crate::{
    kontakt::chunk_set::KontaktChunks,
    nis::{
        properties::{BNISoundPreset, Preset},
        AppSpecific, AuthoringApplication, BNISoundHeader, EncryptionItem, ItemContainer, ItemID,
        RepositoryRoot,
    },
    nks::header::BPatchHeaderV42,
    prelude::*,
    read_bytes::ReadBytesExt,
};
use std::convert::TryFrom;

/// High level wrapper for NISound containers
#[derive(Debug)]
pub struct Repository(ItemContainer);

#[derive(Debug)]
pub enum RepositoryType {
    KontaktPreset,
    AppSpecific,
    Preset,
    Unknown,
}

impl From<ItemContainer> for Repository {
    fn from(ic: ItemContainer) -> Self {
        Self(ic)
    }
}

impl Repository {
    /// Read a NISound repository from a [`std::io::Read`] source.
    ///
    /// ```
    /// use ni_file::NISound;
    ///
    /// let file = std::fs::read("tests/data/nisound/file/fm8/1.2.0.1010/001-fm7.nfm8").unwrap();
    /// let sound = NISound::read(file.as_slice()).unwrap();
    /// ```
    pub fn read<R: ReadBytesExt>(reader: R) -> Result<Self> {
        Ok(Self(ItemContainer::read(reader)?))
    }

    pub fn detect(&self) -> RepositoryType {
        if let Some(child) = self.0.children.get(0) {
            match child.data.header.item_id {
                ItemID::AppSpecific => RepositoryType::AppSpecific,
                ItemID::BNISoundHeader => RepositoryType::KontaktPreset,
                ItemID::Preset => RepositoryType::Preset,
                _ => RepositoryType::Unknown,
            }
        } else {
            RepositoryType::Unknown
        }
    }

    pub fn app_specific(&self) -> Option<Result<AppSpecific>> {
        self.0.find_item(&ItemID::AppSpecific)
    }

    pub fn encryption_item(&self) -> Option<Result<EncryptionItem>> {
        self.0.find_item(&ItemID::EncryptionItem)
    }

    pub fn nks_header(&self) -> Option<Result<BPatchHeaderV42>> {
        self.0
            .find_item::<BNISoundHeader>(&ItemID::BNISoundHeader)
            .map(|sh| sh.map(|h| h.0))
    }

    /// Returns the [`AuthoringApplication`] which created this document.
    pub fn authoring_application(&self) -> Result<AuthoringApplication> {
        // first, lets try find the AppSpecific item
        // (which means this is a multi)
        if let Some(item) = self.0.find_data(&ItemID::AppSpecific) {
            return Ok(AppSpecific::try_from(item)?.authoring_app);
        }

        // not a good way of detecting the authoring app
        // there must be a better solution
        match self.0.find_data(&ItemID::BNISoundPreset) {
            Some(item) => Ok(BNISoundPreset::try_from(item)?.preset.authoring_app),
            None => self
                .0
                .find_data(&ItemID::Preset)
                .and_then(|item_data| Preset::try_from(item_data).ok())
                .map(|preset| preset.authoring_app)
                .ok_or(NIFileError::Generic("not found".to_owned())),
        }

        // match self.authoring_application()? {
        //     AuthoringApplication::Kontakt => self
        //         .0
        //         .find(&ItemID::BNISoundPreset)
        //         .ok_or(NIFileError::Static("Missing chunk: BNISoundPreset"))
        //         .and_then(|item| BNISoundPreset::try_from(item))
        //         .map(|preset| preset.preset),
        //     _ => self
        //         .0
        //         .find(&ItemID::Preset)
        //         .ok_or(NIFileError::Static("Missing chunk: Preset"))
        //         .and_then(|item| Preset::try_from(item))
        //         .map(|preset| preset),
        // }
    }

    /// Returns the version of the embedded preset.
    pub fn preset_version(&self) -> Result<String> {
        // first, lets try find the AppSpecific item
        // (which means this is a multi)
        if let Some(item) = self.0.find_data(&ItemID::AppSpecific) {
            return Ok(AppSpecific::try_from(item)?.version);
        }

        self.preset_item().map(|p| p.version)
    }

    pub fn find_repository_root(&self) -> Option<Result<RepositoryRoot>> {
        self.0.find_item::<RepositoryRoot>(&ItemID::RepositoryRoot)
    }

    /// Get a reference to the underlying [`Item`]. This is switching to the lower level components
    /// that make up the embedded structure of [`NISound`] documents.
    pub fn item(&self) -> &ItemContainer {
        &self.0
    }

    fn preset_item(&self) -> Result<Preset> {
        match self.authoring_application()? {
            AuthoringApplication::Kontakt => self
                .0
                .find_data(&ItemID::BNISoundPreset)
                .ok_or(NIFileError::Static("Missing chunk: BNISoundPreset"))
                .and_then(|item| BNISoundPreset::try_from(item))
                .map(|preset| preset.preset),
            _ => self
                .0
                .find_data(&ItemID::Preset)
                .ok_or(NIFileError::Static("Missing chunk: Preset"))
                .and_then(|item| Preset::try_from(item))
                .map(|preset| preset),
        }
    }

    // pub fn preset(&self) -> Result<PresetContainer> {
    //     self.encryption_item()
    //         .and_then(|item| PresetContainer::try_from(ItemData::read(Cursor::new(item))?))
    // }

    pub fn children(&self) -> &Vec<ItemContainer> {
        &self.0.children
    }

    pub fn instrument(&self) -> Result<KontaktChunks> {
        todo!()
    }

    // pub fn preset(&self) -> Result<PresetContainer> {
    //     todo!()
    // }

    // pub fn instrument(&self) -> Option<Result<KontaktInstrument>> {
    //     if let Some(Ok(item)) = self.encryption_item() {
    //         let preset_container = item.subtree.item()?;
    //         let data = preset_container.inner().data;
    //         let item = ItemContainer::read(Cursor::new(data))?;
    //         match item.find(&ItemID::PresetChunkItem) {
    //             Some(preset_item_frame) => {
    //                 let preset_chunk_item: PresetChunkItem =
    //                     preset_item_frame.clone().try_into()?;
    //                 let data = preset_chunk_item.chunk();
    //
    //                 match self.authoring_application()? {
    //                     AuthoringApplication::Kontakt => {
    //                         let mut objects = Vec::new();
    //                         let mut compressed_data = Cursor::new(&data);
    //
    //                         while let Ok(chunk) = Chunk::read(&mut compressed_data) {
    //                             objects.push(chunk);
    //                         }
    //
    //                         Some(Ok(KontaktInstrument(objects)))
    //                     }
    //                     _ => todo!(),
    //                 }
    //             }
    //             None => todo!(),
    //         }
    //     }
    //     None
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_container_read_test() -> Result<()> {
        let file = std::fs::File::open("tests/filetype/NISD/kontakt/7.1.3.0/000-default.nki")?;
        let repository = Repository::read(file)?;
        repository.find_repository_root().unwrap()?;
        Ok(())
    }
}
