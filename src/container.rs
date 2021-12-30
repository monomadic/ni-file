// pub struct NIFile {
//     data
// }

pub struct NISegment {
    id: u32,
    data: Vec<u8>,
}

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
            101 => SegmentType::Maybe("Massive File".into()),
            _ => SegmentType::Unknown(id),
        }
    }
}
