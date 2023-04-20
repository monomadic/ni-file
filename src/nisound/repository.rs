use super::{
    item::Item,
    item_frame::item_id::ItemID,
    items::{AppSpecific, EncryptionItem, RepositoryRoot, RepositoryVersion},
    AuthoringApplication,
};
use crate::{
    nisound::items::{BNISoundPreset, Preset, PresetChunkItem},
    prelude::*,
    read_bytes::ReadBytesExt,
};
use std::convert::{TryFrom, TryInto};

/// High level wrapper for NISound containers. As this file format is very complex, this wrapper
/// was created for most users. Unless you are exploring unknown parts of the standards NI created,
/// this is probably the way you want to use this library.
pub struct NISound(Item);

impl NISound {
    /// Read a NISound repository from a [`std::io::Read`] source.
    ///
    /// ```
    /// use ni_file::NISound;
    ///
    /// let file = std::fs::read("tests/data/nisound/file/fm8/1.2.0.1010/001-fm7.nfm8").unwrap();
    /// let sound = NISound::read(file.as_slice()).unwrap();
    /// ```
    pub fn read<R: ReadBytesExt>(reader: R) -> Result<Self> {
        log::debug!("NISound::read()");
        Ok(Self(Item::read(reader)?))
    }

    /// Returns the [`RepositoryVersion`], also referred to sometimes as the NISD Version.
    pub fn nisound_version(&self) -> Result<RepositoryVersion> {
        RepositoryRoot::try_from(self.0.data()?).map(|root| root.version())
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
        RepositoryRoot::try_from(self.0.data()?)
    }

    /// Get a reference to the underlying [`Item`]. This is switching to the lower level components
    /// that make up the embedded structure of [`NISound`] documents.
    pub fn item(&self) -> &Item {
        &self.0
    }

    /// Inner preset chunk.
    pub fn chunk(&self) -> Result<Vec<u8>> {
        let inner = Item::read(self.inner_container()?.as_slice())?;
        let data = inner.children[0].data()?;
        let chunk_item = PresetChunkItem::try_from(data)?;

        // TODO: lifetime?
        Ok(chunk_item.chunk().clone())
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

    // TODO: called InternalPatchData
    pub fn inner_container(&self) -> Result<Vec<u8>> {
        let item = self
            .0
            .find(&ItemID::EncryptionItem)
            .expect("no EncryptionItem");
        let ei: EncryptionItem = item.try_into()?;
        Ok(ei.subtree.inner_data)
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

        let repo = NISound::read(
            include_bytes!("../../tests/data/nisound/file/kontakt/7.1.3.0/000-default.nki")
                .as_slice(),
        )?;
        let _root = repo.root()?;

        // TODO: repo props

        Ok(())
    }
}
