// pub struct NIContainer {
//     data: NIDataSegment,
//     children: Node<NIContainer>
// }

// pub struct NISegment {
//     id: u32,
//     data: Vec<u8>,
// }

#[derive(Debug)]
pub enum SegmentType {
    FileHeader,
    Maybe(String),
    Unknown(u32),
}

impl From<u32> for SegmentType {
    fn from(id: u32) -> Self {
        match id {
            118 => SegmentType::FileHeader,
            3 => SegmentType::Maybe("Kontakt File".into()),
            101 => SegmentType::Maybe("Container?".into()),
            108 => SegmentType::Maybe("Container Part 1".into()),
            121 => SegmentType::Maybe("Container Part 2".into()),
            116 => SegmentType::Maybe("Container Part 3".into()),
            _ => SegmentType::Unknown(id),
        }
    }
}
