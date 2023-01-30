use binread::BinRead;

#[derive(BinRead, Debug, Clone)]
struct Frame {
    size: u64,
    #[br(count = size, seek_before=std::io::SeekFrom::Current(-8))]
    data: Vec<u8>,
}

pub fn read(mut buffer: &[u8]) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading_frame_from_files() {}
}
