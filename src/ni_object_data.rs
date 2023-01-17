use crate::kinds::SegmentType;

pub enum NIObjectData {
    Unparsed(u32),
}

pub fn parse(ty: SegmentType, _data: &[u8]) -> NIObjectData {
    match ty {
        SegmentType::AppSpecific => todo!(),
        SegmentType::AudioSampleItem => todo!(),
        SegmentType::Authorization => todo!(),
        SegmentType::AutomationParameters => todo!(),
        SegmentType::Bank => todo!(),
        SegmentType::BankContainer => todo!(),
        SegmentType::BinaryChunkItem => todo!(),
        SegmentType::BNISoundHeader => todo!(),
        SegmentType::ControllerAssignments => todo!(),
        SegmentType::EncryptionItem => todo!(),
        SegmentType::ExternalFileReference => todo!(),
        SegmentType::GenericItem(_) => todo!(),
        SegmentType::InternalResourceReferenceItem => todo!(),
        SegmentType::Item => todo!(),
        SegmentType::Module => todo!(),
        SegmentType::ModuleBank => todo!(),
        SegmentType::PictureItem => todo!(),
        SegmentType::Preset => todo!(),
        SegmentType::PresetChunkItem => todo!(),
        SegmentType::PresetContainer => todo!(),
        SegmentType::PresetInner => todo!(),
        SegmentType::RepositoryRoot => todo!(),
        SegmentType::Resources => todo!(),
        SegmentType::SoundInfoItem => todo!(),
        SegmentType::SubtreeItem => todo!(),
        SegmentType::Unknown(id) => NIObjectData::Unparsed(id),
    }
}
