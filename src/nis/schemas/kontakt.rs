use std::io::Cursor;

use crate::{
    kontakt::{KontaktInstrument, KontaktPreset},
    nis::{
        BNISoundHeader, BNISoundPreset, EncryptionItem, ItemContainer, ItemID, Preset,
        PresetChunkItem,
    },
    nks::header::{BPatchHeader, BPatchHeaderV42},
    Error, NIFileError,
};

impl ItemContainer {
    pub fn find_kontakt_preset_item(&self) -> Option<Result<Preset, Error>> {
        if let Ok(b) = self.find_item::<BNISoundPreset>(&ItemID::BNISoundPreset)? {
            return Some(Ok(b.preset));
        }
        None
    }

    pub fn find_kontakt_header(&self) -> Option<Result<BPatchHeaderV42, Error>> {
        self.find_item::<BNISoundHeader>(&ItemID::BNISoundHeader)
            .map(|r| r.map(|h| h.0))
    }

    pub fn find_encryption_item(&self) -> Option<Result<EncryptionItem, Error>> {
        self.find_data(&ItemID::EncryptionItem)
            .map(EncryptionItem::try_from)
    }

    pub fn extract_kontakt_preset(&self) -> Option<Result<KontaktInstrument, Error>> {
        let header = match self.find_kontakt_header()? {
            Ok(header) => header,
            Err(e) => return Some(Err(e)),
        };

        let preset: KontaktPreset = match self.find_encryption_item()? {
            Ok(enc) => {
                let item = enc.subtree.item().unwrap();

                match item.find_item::<PresetChunkItem>(&ItemID::PresetChunkItem) {
                    Some(preset_chunk_item) => {
                        let chunk = preset_chunk_item.unwrap();
                        let chunk = chunk.chunk();
                        KontaktPreset::from_str(&mut Cursor::new(&chunk), &header.app_signature)
                            .unwrap()
                    }
                    None => todo!(),
                }
            }
            Err(e) => return Some(Err(e)),
        };

        Some(Ok(KontaktInstrument {
            header: BPatchHeader::BPatchHeaderV42(header),
            preset,
        }))
    }
}

#[derive(Debug)]
pub struct BNISoundPresetContainer(ItemContainer);

impl BNISoundPresetContainer {
    pub fn properties(&self) -> Result<BNISoundPreset, Error> {
        BNISoundPreset::try_from(&self.0.data)
    }

    pub fn header(&self) -> Option<Result<BNISoundHeader, Error>> {
        self.0
            .find_data(&ItemID::BNISoundHeader)
            .map(|item_data| BNISoundHeader::try_from(item_data))
    }
}

impl TryFrom<&ItemContainer> for BNISoundPresetContainer {
    type Error = Error;

    fn try_from(container: &ItemContainer) -> Result<Self, Self::Error> {
        let id = container.id();
        if id != &ItemID::BNISoundPreset {
            return Err(Error::ItemWrapError {
                expected: ItemID::BNISoundPreset,
                got: id.clone(),
            });
        }
        Ok(Self(container.clone()))
    }
}
