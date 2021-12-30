// pub struct NIContainer {
//     data: NIDataSegment,
//     children: Node<NIContainer>
// }

// pub struct NISegment {
//     id: u32,
//     data: Vec<u8>,
// }

#[derive(Debug, Clone)]
pub enum SegmentType {
    FileHeader,
    Version,
    LibraryMetadata,
    CompressedSegment,
    Maybe(String),
    Unknown(u32),
}

impl From<u32> for SegmentType {
    fn from(id: u32) -> Self {
        match id {
            3 => SegmentType::Maybe("KontaktFile".into()),
            101 => SegmentType::Version,
            108 => SegmentType::LibraryMetadata,
            115 => SegmentType::CompressedSegment,
            118 => SegmentType::FileHeader,
            121 => SegmentType::Maybe("ContainerPart2".into()),
            116 => SegmentType::Maybe("ContainerPart3".into()),
            _ => SegmentType::Unknown(id),
        }
    }
}
