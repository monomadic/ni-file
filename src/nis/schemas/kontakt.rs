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
pub struct KontaktPresetSchema(ItemContainer);

impl From<&ItemContainer> for KontaktPresetSchema {
    fn from(ic: &ItemContainer) -> Self {
        Self(ic.clone())
    }
}

impl KontaktPresetSchema {
    pub fn header(&self) -> Result<BPatchHeaderV42, NIFileError> {
        self.0
            .find_data(&ItemID::BNISoundHeader)
            .ok_or(NIFileError::Static("missing"))
            .and_then(|item_data| BNISoundHeader::try_from(item_data).map(|sh| sh.0))
    }

    pub fn preset_item(&self) -> Result<Preset, NIFileError> {
        self.0
            .find_data(&ItemID::BNISoundPreset)
            .ok_or(NIFileError::Static("missing"))
            .and_then(|item_data| BNISoundPreset::try_from(item_data).map(|sh| sh.preset))
    }

    pub fn preset_data(&self) -> Result<Vec<u8>, NIFileError> {
        self.0
            .find_data(&ItemID::EncryptionItem)
            .ok_or(NIFileError::Static("missing"))
            .and_then(|item_data| {
                EncryptionItem::try_from(item_data).map(|sh| sh.subtree.inner_data)
            })
    }

    // pub fn sound_info(&self) -> Result<SoundInfoItem, NIFileError> {
    //     self.0
    //         .find(&ItemID::BNISoundHeader)
    //         .ok_or(NIFileError::Static("missing"))
    //         .and_then(|item_data| BNISoundHeader::try_from(item_data).map(|sh| sh.0))
    // }
}
