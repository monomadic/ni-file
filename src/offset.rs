/// get offsets from the dictionary (which is previously decompressed data)

pub(crate) fn fetch_offset(buffer: &Vec<u8>, length: usize, offset: usize) -> Vec<u8> {
    // let mut buffer: Vec<u8> = buffer.clone();
    // buffer.reverse();

    // let start = offset;
    // let end = offset + length;

    // assert!(start < end);

    // let mut slice = buffer[start..end].to_vec();
    // slice.reverse();

    // slice

    let start = buffer.len() - offset;
    let end = start + length;

    buffer[start..end].to_vec()
}

#[test]
fn test_fetch_offset() {
    assert_eq!(fetch_offset(&vec![0x00, 0x00, 0x00, 0x54, 0x45, 0x53, 0x54, 0x53, 0x54, 0x52, 0x49, 0x4E, 0x47, 0x31, 0x17], 3, 15), vec![0x00, 0x00, 0x00]);
    assert_eq!(fetch_offset(&vec![0x00, 0x00, 0x00, 0xF9, 0x15, 0x00], 5, 1), vec![0x00]);
}