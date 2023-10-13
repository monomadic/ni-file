/// ID representing an Item type.
#[derive(PartialEq, Debug, Clone)]
pub enum ItemType {
    AppSpecific,
    AudioSampleItem,
    Authorization,
    AutomationParameters,
    Bank,
    BankContainer,
    BinaryChunkItem,
    BNISoundHeader,
    BNISoundPreset,
    ControllerAssignments,
    EncryptionItem,
    ExternalFileReference,
    InternalResourceReferenceItem,
    Item,
    Module,
    ModuleBank,
    PictureItem,
    Preset,
    PresetChunkItem,
    PresetContainer,
    PresetInner,
    RepositoryRoot,
    Resources,
    SoundInfoItem,
    SubtreeItem,
    Unknown(u32, String),
}

impl ItemType {
    pub fn new(item_id: u32, domain_id: &str) -> Self {
        match domain_id {
            "NISD" => {
                match item_id {
                    0x01 => ItemType::Item,
                    0x64 => ItemType::Bank,
                    0x65 => ItemType::Preset,
                    0x66 => ItemType::BankContainer,
                    0x67 => ItemType::PresetContainer,
                    0x68 => ItemType::BinaryChunkItem,
                    0x6a => ItemType::Authorization,
                    0x6c => ItemType::SoundInfoItem,
                    0x6d => ItemType::PresetChunkItem, // occurs in first header of deflated kontakt
                    0x6e => ItemType::ExternalFileReference,
                    0x6f => ItemType::Resources,
                    0x70 => ItemType::AudioSampleItem,
                    0x71 => ItemType::InternalResourceReferenceItem,
                    0x72 => ItemType::PictureItem,
                    0x73 => ItemType::SubtreeItem,
                    0x74 => ItemType::EncryptionItem,
                    0x75 => ItemType::AppSpecific,
                    0x76 => ItemType::RepositoryRoot,
                    0x78 => ItemType::AutomationParameters,
                    0x79 => ItemType::ControllerAssignments,
                    0x7a => ItemType::Module,
                    0x7b => ItemType::ModuleBank,
                    _ => ItemType::Unknown(item_id, domain_id.into()),
                }
            }
            "NIK4" => {
                match item_id {
                    0x03 => ItemType::BNISoundPreset, // domainID NIK4 only
                    0x04 => ItemType::BNISoundHeader, // NIK4 only
                    _ => ItemType::Unknown(item_id, domain_id.into()),
                }
            }
            _ => ItemType::Unknown(item_id, domain_id.into()),
        }
    }
}
