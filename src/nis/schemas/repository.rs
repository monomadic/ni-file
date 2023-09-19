use super::preset_container::PresetContainer;
use crate::{
    kontakt::{chunkdata::ChunkData, instrument::KontaktInstrument},
    nis::{
        properties::{BNISoundPreset, Preset, PresetChunkItem},
        AppSpecific, AuthoringApplication, BNISoundHeader, EncryptionItem, ItemContainer, ItemData,
        ItemID, RepositoryRoot, RepositoryVersion,
    },
    nks::header::BPatchHeaderV42,
    prelude::*,
    read_bytes::ReadBytesExt,
};
use std::{convert::TryFrom, io::Cursor};

/// High level wrapper for NISound containers
pub struct Repository(ItemContainer);

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

    pub fn nks_header(&self) -> Result<BPatchHeaderV42> {
        if let Some(item) = self.0.find(&ItemID::BNISoundHeader) {
            let reader = Cursor::new(&item.data);
            return Ok(BNISoundHeader::read(reader)?.0);
        }
        Err(NIFileError::Static("could not find BNISoundHeader"))
    }

    /// Returns the [`RepositoryVersion`], also referred to sometimes as the NISD Version.
    pub fn nisound_version(&self) -> Result<RepositoryVersion> {
        RepositoryRoot::try_from(&self.0.data).map(|root| root.version())
    }

    /// Returns the [`AuthoringApplication`] which created this document.
    pub fn authoring_application(&self) -> Result<AuthoringApplication> {
        // first, lets try find the AppSpecific item
        // (which means this is a multi)
        if let Some(item) = self.0.find(&ItemID::AppSpecific) {
            return Ok(AppSpecific::try_from(item)?.authoring_app);
        }

        // not a good way of detecting the authoring app
        // there must be a better solution
        match self.0.find(&ItemID::BNISoundPreset) {
            Some(item) => Ok(BNISoundPreset::try_from(item)?.preset.authoring_app),
            None => self
                .0
                .find(&ItemID::Preset)
                .and_then(|item_frame| Preset::try_from(item_frame).ok())
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
        if let Some(item) = self.0.find(&ItemID::AppSpecific) {
            return Ok(AppSpecific::try_from(item)?.version);
        }

        self.preset_item().map(|p| p.version)
    }

    pub fn root(&self) -> Result<RepositoryRoot> {
        RepositoryRoot::try_from(&self.0.data)
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
                .find(&ItemID::BNISoundPreset)
                .ok_or(NIFileError::Static("Missing chunk: BNISoundPreset"))
                .and_then(|item| BNISoundPreset::try_from(item))
                .map(|preset| preset.preset),
            _ => self
                .0
                .find(&ItemID::Preset)
                .ok_or(NIFileError::Static("Missing chunk: Preset"))
                .and_then(|item| Preset::try_from(item))
                .map(|preset| preset),
        }
    }

    pub fn preset_raw(&self) -> Result<Vec<u8>> {
        self.0
            .find(&ItemID::EncryptionItem)
            .ok_or(NIFileError::Generic(format!("EncryptionItem not found")))
            .and_then(|item_frame| EncryptionItem::try_from(item_frame))
            .map(|item| item.subtree.inner_data)
    }

    pub fn preset(&self) -> Result<PresetContainer> {
        self.preset_raw()
            .and_then(|item| PresetContainer::try_from(ItemData::read(Cursor::new(item))?))
    }

    pub fn children(&self) -> &Vec<ItemContainer> {
        &self.0.children
    }

    pub fn instrument(&self) -> Result<KontaktInstrument> {
        let preset = self.preset_raw()?;
        let item = ItemContainer::read(Cursor::new(preset))?;
        match item.find(&ItemID::PresetChunkItem) {
            Some(preset_item_frame) => {
                let preset_chunk_item: PresetChunkItem = preset_item_frame.clone().try_into()?;
                let data = preset_chunk_item.chunk();

                match self.authoring_application()? {
                    AuthoringApplication::Kontakt => {
                        let mut objects = Vec::new();
                        let mut compressed_data = Cursor::new(&data);

                        while let Ok(chunk) = ChunkData::read(&mut compressed_data) {
                            objects.push(chunk);
                        }

                        Ok(KontaktInstrument(objects))
                    }
                    _ => todo!(),
                }
            }
            None => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_container_read_test() -> Result<()> {
        let file = std::fs::File::open("tests/filetype/NISD/kontakt/7.1.3.0/000-default.nki")?;
        let repository = Repository::read(file)?;
        repository.root()?;
        Ok(())
    }
}
