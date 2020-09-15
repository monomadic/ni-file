/// get offsets from the dictionary (which is previously decompressed data)

pub(crate) fn fetch_offset(buffer: &Vec<u8>, length: usize, offset: usize) -> Vec<u8> {
    if offset > buffer.len() {
        return vec![];
        panic!("Cannot deflate: offset seek is larger than dictionary.");
    }

    (0..length)
        .map(|index| {
            let start_pos = buffer.len() - offset;
            let offset_pos = start_pos + index;

            if length > offset {
                // let circular_pos = offset_pos - offset;
                // panic!("attempt {:?}",  (buffer.len(), offset, index));
                let circular_pos = start_pos + (index % offset);
                if circular_pos > buffer.len() {
                    panic!("attempt {:?}", (circular_pos, offset, buffer.len()));
                }
                buffer[circular_pos]
            } else {
                buffer[offset_pos]
            }

        })
        .collect()

    // if offset > buffer.len() {
    //     println!("ERROR: buffer underrun {} {}", length, offset);
    //     return vec![];
    // }

    // let start = buffer.len() - offset;
    // let end = start + length;

    // if end > buffer.len() || start > buffer.len() {
    //     println!("ERROR: bad size {} {}", length, offset);
    //     return vec![];
    // }

    // buffer[start..end].to_vec()
}

#[test]
fn test_fetch_offset() {
    assert_eq!(
        fetch_offset(
            &vec![
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07],
            3,
            7
        ),
        vec![0x01, 0x02, 0x03]
    );

    assert_eq!(
        fetch_offset(&vec![0x01, 0x02, 0x03, 0xF4, 0x15, 0x06], 1, 5),
        vec![0x02]
    );

    assert_eq!(
        fetch_offset(&vec![0x00, 0x01, 0x00, 0x00, 0x00], 16, 4),
        vec![0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00]
    );

    assert_eq!(
        fetch_offset(&vec![0x01, 0x02, 0xF4, 0x08, 0x00], 3, 1),
        vec![0x00, 0x00, 0x00]
    );
}
