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
                    1 => ItemType::Item,
                    100 => ItemType::Bank,
                    101 => ItemType::Preset,
                    102 => ItemType::BankContainer,
                    103 => ItemType::PresetContainer,
                    104 => ItemType::BinaryChunkItem,
                    106 => ItemType::Authorization,
                    108 => ItemType::SoundInfoItem,
                    109 => ItemType::PresetChunkItem, // occurs in first header of deflated kontakt
                    110 => ItemType::ExternalFileReference,
                    111 => ItemType::Resources,
                    112 => ItemType::AudioSampleItem,
                    113 => ItemType::InternalResourceReferenceItem,
                    114 => ItemType::PictureItem,
                    115 => ItemType::SubtreeItem,
                    116 => ItemType::EncryptionItem,
                    117 => ItemType::AppSpecific,
                    118 => ItemType::RepositoryRoot, // 0x76
                    120 => ItemType::AutomationParameters,
                    121 => ItemType::ControllerAssignments,
                    122 => ItemType::Module,
                    123 => ItemType::ModuleBank,
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
