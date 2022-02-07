#[test]
fn test_container_parser() {
    for file in glob::glob_with("./tests/data/ni_container/*.*", glob::MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    }).unwrap() {
        let path = format!("{:?}", &file);
        let buffer = std::fs::read(file.unwrap()).unwrap();
        let container = ni_file::ni_container::read(&buffer);  

        assert!(container.is_ok(), "reading container {}", path);
        let container = container.unwrap();

        assert_eq!(container.length, buffer.len() as u64, "container.length {}", path);
        assert_eq!(container.unknown_a, 1);
        assert_eq!(container.tag, ['h', 's', 'i', 'n']);
        assert_eq!(container.id, 1);

        assert_eq!(container.data_len, container.data_chunk.len() as u32);
        
        assert_eq!(container.current_index, 1);
        assert_eq!(container.children_length, container.children.len() as u32);
    }
}

#[test]
fn test_kontakt_4_booga() {
    let file = include_bytes!("./data/ni_container/kontakt-4--booga.nki");
    let container = ni_file::ni_container::read(file).unwrap();

    assert_eq!(container.length, file.len() as u64);
    assert_eq!(container.unknown_a, 1);
    assert_eq!(container.current_index, 1);
    assert_eq!(&container.tag(), "hsin");
    assert_eq!(container.id, 1);

    assert_eq!(container.data_len, container.data_chunk.len() as u32);
    assert_eq!(container.children_length, 1);
    assert_eq!(container.current_index, 1);

    assert_eq!(container.children[0].chunk.tag, ['h', 's', 'i', 'n']);
}

#[test]
fn test_k5_4_demo() {
    let file = include_bytes!("./data/ni_container/kontakt-5.4-demo.nki");
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
    assert_eq!(container.children_length, 1);
    assert_eq!(container.to_string(), "<hsin></hsin>");
}

#[test]
fn test_fm8_fm7() {
    let file = include_bytes!("./data/ni_container/fm8--fm7.nfm8");
    let container = ni_file::ni_container::read(file).unwrap();

    assert_eq!(container.length, file.len() as u64);
    assert_eq!(container.unknown_a, 1);
    assert_eq!(&container.tag(), "hsin");
    assert_eq!(container.id, 1);
    assert_eq!(container.data_len, container.data_chunk.len() as u32);
    assert_eq!(container.children_length, 1);
}

#[test]
fn test_massive_mexp() {
    let file = include_bytes!("./data/ni_container/massive--mexp.nmsv");
    let container = ni_file::ni_container::read(file).unwrap();
}

#[test]
fn test_maschine_standard_2() {
    let file = include_bytes!("./data/ni_container/maschine--standard-2.mxfx");
    let container = ni_file::ni_container::read(file).unwrap();
}

#[test]
fn test_guitar_rig_rammfire() {
    let file = include_bytes!("./data/ni_container/guitar-rig--rammfire.ngrr");
    let container = ni_file::ni_container::read(file).unwrap();

    assert_eq!(container.length, file.len() as u64);
    assert_eq!(container.unknown_a, 1);
    assert_eq!(&container.tag(), "hsin");
    assert_eq!(container.id, 1);
    assert_eq!(container.data_len, container.data_chunk.len() as u32);
    assert_eq!(container.children_length, 2);
}
