/// get offsets from the dictionary (which is previously decompressed data)

pub(crate) fn fetch_offset(buffer: &Vec<u8>, offset: usize, length: usize) -> Vec<u8> {
    let mut buffer: Vec<u8> = buffer.clone();
    buffer.reverse();

    let start = offset - length;
    let end = offset;

    assert!(start < end);

    let mut slice = buffer[start..end].to_vec();
    slice.reverse();

    slice
}

#[test]
fn test_fetch_offset() {
    // assert_eq!(fetch_offset(&vec![11, 22, 33, 44, 55], 5, 2), vec![11, 22]);
    assert_eq!(fetch_offset(&vec![0x00, 0x00, 0x00, 0x54, 0x45, 0x53, 0x54, 0x53, 0x54, 0x52, 0x49, 0x4E, 0x47, 0x31, 0x17 ], 15, 3), vec![0x00, 0x00, 0x00]);
}