use super::{
    item::Item,
    item_frame::item_id::ItemID,
    items::{encryption_item::EncryptionItem, RepositoryRoot, RepositoryVersion},
    AuthoringApplication,
};
use crate::{
    nisound::items::{bni_sound_preset::BNISoundPreset, Preset, PresetChunkItem},
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
    pub fn version(&self) -> Result<RepositoryVersion> {
        self.0
            .find(&ItemID::RepositoryRoot)
            .ok_or(NIFileError::Static("Missing RepositoryRoot"))
            .and_then(|item| RepositoryRoot::try_from(item))
            .map(|root| root.version())
    }

    /// Returns the [`AuthoringApplication`] which created this document.
    pub fn authoring_application(&self) -> Option<AuthoringApplication> {
        self.0
            .find(&ItemID::Preset)
            .and_then(|item| Preset::try_from(item).ok())
            .map(|preset| preset.authoring_app)
    }

    /// Returns the version of the embedded preset.
    pub fn preset_version(&self) -> Option<String> {
        self.0
            .find(&ItemID::Preset)
            .and_then(|item| Preset::try_from(item).ok())
            .map(|preset| preset.version)
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

    pub fn preset(&self) -> Result<Preset> {
        for item in &self.0.children {
            match item.data()?.header.item_id {
                ItemID::BNISoundPreset => {
                    // TODO: cleaner
                    return Ok(BNISoundPreset::try_from(item.data()?)?.preset);
                }
                ItemID::Preset => return Ok(Preset::try_from(item.data()?)?),
                _ => todo!(),
            }
        }

        todo!()
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
