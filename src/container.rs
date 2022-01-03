use rctree::Node;

pub struct NIHeaderSegment {
    data: NIDataSegment,
    children: Node<NIDataSegment>,
}

pub struct NIDataSegment {
    id: u32,
    data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub enum SegmentType {
    FileHeader,
    Version,
    LibraryMetadata,
    CompressedPreset,
    PresetContainer,
    PresetInner,
    Maybe(String),
    Unknown(u32),
}

impl From<u32> for SegmentType {
    fn from(id: u32) -> Self {
        match id {
            1 => SegmentType::CompressedPreset,
            3 => SegmentType::Maybe("KontaktFile".into()),
            101 => SegmentType::Version,
            108 => SegmentType::LibraryMetadata,
            115 => SegmentType::PresetInner,
            116 => SegmentType::PresetContainer,
            118 => SegmentType::FileHeader,
            121 => SegmentType::Maybe("ContainerPart2".into()),
            _ => SegmentType::Unknown(id),
        }
    }
}
