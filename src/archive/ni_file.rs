use crate::extract::NISegment;

pub struct NIFile {
    pub preset: Vec<u8>,
}

impl From<NISegment<'_>> for NIFile {
    fn from(segment: NISegment) -> NIFile {
        NIFile { preset: Vec::new() }
    }
}
