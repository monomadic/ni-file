use binread::{io::Cursor, prelude::*, NullWideString};
use std::io::prelude::*;

#[test]
fn test_container_parser() {
    for file in glob::glob_with("./tests/data/ni_container/*.*", glob::MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    }).unwrap() {
        let path = file.as_ref().unwrap();
        println!("testing {:?}", &path);

        let buffer = std::fs::read(path).unwrap();
        let container = ni_file::ni_container::read(&buffer);  

        assert!(container.is_ok(), "reading container {:?}", path);
        let container = container.unwrap();

        assert_eq!(container.length, buffer.len() as u64, "container.length {:?}", path);
        assert_eq!(container.unknown_a, 1);
        assert_eq!(container.tag, ['h', 's', 'i', 'n']);
        assert_eq!(container.id, 1);
        // assert_eq!(container.data_len, container.data_chunk.len() as u32);
        assert_eq!(container.current_index, 1, "current_index in {:?}", path);
        assert_eq!(container.children_length, container.children.len() as u32);



        let mut cursor = Cursor::new(&container.data_chunk);
        // let data_segment: ni_file::ni_container::DataChunk = cursor.read_le().unwrap();

        // println!("{:?}", data_segment);

        for child in container.children {
            // println!("{}", child.chunk.tag());
            for child in child.chunk.children {
                if child.chunk.children_length > 0 {
                    println!("fourth level chunk found {}", child.chunk.children_length);
                    for child in child.chunk.children {
                        if child.chunk.children_length > 0 {
                            println!("fifth level chunk found {}", child.chunk.children_length);
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn test_kontakt_4_booga() {
    let file = include_bytes!("./data/ni_container/kontakt-4--booga.nki");
    let container = ni_file::ni_container::read(file).unwrap();
}

#[test]
fn test_k5_4_demo() {
    // checksums:
    // u32le;4  0d881716e742958f4695d6978a38bca0
    // u16le;8  17160d88958fe742d6974695bca08a38
    // u8;16    1617880d8f9542e797d69546a0bc388a

    let file = include_bytes!("./data/ni_container/kontakt-5.4-demo.nki");
    let container = ni_file::ni_container::read(file).unwrap(); 
}

#[test]
fn test_fm8_fm7() {
    let file = include_bytes!("./data/ni_container/fm8--fm7.nfm8");
    let container = ni_file::ni_container::read(file).unwrap();
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

// #[test]
// fn test_guitar_rig_rammfire() {
//     let file = include_bytes!("./data/ni_container/guitar-rig--rammfire.ngrr");
//     let container = ni_file::ni_container::read(file).unwrap();
// }
