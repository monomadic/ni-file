#[test]
fn test_kontakt_4_booga() {
    let file = include_bytes!("./data/k4-booga.nki");
    let container = ni_file::ni_container::read(file).unwrap();

    assert_eq!(container.length, file.len() as u64);
    assert_eq!(container.unknown_a, 1);
    assert_eq!(container.unknown_b, 1);
    assert_eq!(&container.tag(), "hsin");
    assert_eq!(container.id, 1);

    assert_eq!(container.data_len - 4, container.data_chunk.len() as u32);
    assert_eq!(container.children, 1);
    assert_eq!(container.unknown_b, 1);
    // assert_eq!(container.inner_length - 8, container.inner_chunk.len() as u64);
}

#[test]
fn test_k5_4_demo() {
    let file = include_bytes!("./data/k5.4-demo.nki");
    let container = ni_file::ni_container::read(file).unwrap();

    assert_eq!(container.length, file.len() as u64);
    assert_eq!(container.unknown_a, 1);
    assert_eq!(&container.tag(), "hsin");
    assert_eq!(container.id, 1);
    // assert_eq!(&container.checksum(), "444");
    // checksums:
    // u32le;4  0d881716e742958f4695d6978a38bca0
    // u16le;8  17160d88958fe742d6974695bca08a38 
    // u8;16    1617880d8f9542e797d69546a0bc388a

    assert_eq!(container.data_len, container.data_chunk.len() as u32);
    assert_eq!(container.children, 1);
    assert_eq!(container.siblings, 0);
    assert_eq!(container.unknown_b, 1);

    assert_eq!(container.inner_id, 3); //4kin:3

    // assert_eq!(container.inner_length - 8, container.inner_chunk.len() as u64);


    // second inner segment
    let container = ni_file::ni_container::read(&container.inner_chunk).unwrap();

    assert_eq!(container.to_string(), "<hsin></hsin>");
    // assert_eq!("", format!("{:?}", container));
}

#[test]
fn test_fm8_fm7() {
    let file = include_bytes!("./data/fm8-fm7.nfm8");
    let container = ni_file::ni_container::read(file).unwrap();

    assert_eq!(container.length, file.len() as u64);
    assert_eq!(container.unknown_a, 1);
    assert_eq!(container.unknown_b, 1);
    assert_eq!(&container.tag(), "hsin");
    assert_eq!(container.id, 1);

    assert_eq!(container.data_len - 4, container.data_chunk.len() as u32);
    assert_eq!(container.children, 1);
    assert_eq!(container.unknown_b, 1);
    // assert_eq!(container.inner_length - 8, container.inner_chunk.len() as u64);
}
