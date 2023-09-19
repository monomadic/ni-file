/// ID representing an Item type.
#[derive(PartialEq, Debug, Clone)]
pub enum ItemID {
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
    Unknown(u32),
}

impl From<u32> for ItemID {
    fn from(id: u32) -> Self {
        match id {
            1 => ItemID::Item,
            3 => ItemID::BNISoundPreset, // domainID NIK4 only
            4 => ItemID::BNISoundHeader, // NIK4 only
            100 => ItemID::Bank,
            101 => ItemID::Preset,
            102 => ItemID::BankContainer,
            103 => ItemID::PresetContainer,
            104 => ItemID::BinaryChunkItem,
            106 => ItemID::Authorization,
            108 => ItemID::SoundInfoItem,
            109 => ItemID::PresetChunkItem, // occurs in first header of deflated kontakt
            110 => ItemID::ExternalFileReference,
            111 => ItemID::Resources,
            112 => ItemID::AudioSampleItem,
            113 => ItemID::InternalResourceReferenceItem,
            114 => ItemID::PictureItem,
            115 => ItemID::SubtreeItem,
            116 => ItemID::EncryptionItem,
            117 => ItemID::AppSpecific,
            118 => ItemID::RepositoryRoot, // 0x76
            120 => ItemID::AutomationParameters,
            121 => ItemID::ControllerAssignments,
            122 => ItemID::Module,
            123 => ItemID::ModuleBank,
            _ => ItemID::Unknown(id),
        }
    }
}
