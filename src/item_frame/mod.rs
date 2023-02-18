mod preset;
pub(crate) mod repository_root;

#[derive(PartialEq, Debug, Clone, Default)]
pub enum ItemType {
    AppSpecific,
    AudioSampleItem,
    Authorization,
    AutomationParameters,
    Bank,
    BankContainer,
    BinaryChunkItem,
    BNISoundHeader,
    ControllerAssignments,
    EncryptionItem,
    ExternalFileReference,
    GenericItem(Box<ItemType>),
    InternalResourceReferenceItem,
    Item,
    Module,
    ModuleBank,
    PictureItem,
    Preset,
    PresetChunkItem,
    PresetContainer,
    PresetInner,
    #[default]
    RepositoryRoot,
    Resources,
    SoundInfoItem,
    SubtreeItem,
    Unknown(u32),
}

impl From<u32> for ItemType {
    fn from(id: u32) -> Self {
        match id {
            // ItemFrame
            1 => ItemType::Item,
            3 => ItemType::BNISoundHeader,
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
            // ?? => SegmentType::Container,
            _ => ItemType::Unknown(id),
        }
    }
}
