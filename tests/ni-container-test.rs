#[test]
fn test_kontakt() {
    // let file = include_bytes!("./data/ni_container/kontakt-4--booga.nki");
    let file = include_bytes!("./data/ni_container/kontakt-5.4-demo.nki");
    let container = ni_file::ni_container::NIContainer::read(file).expect("file to read");
    assert_eq!(
        container.to_xml(),
        include_str!("./output/kontakt-5.4-demo.xml")
    );

    let container = container.data;
    assert_eq!(container.children_length, 1);

    let unknown_segment = &container.children[0];
    assert_eq!(unknown_segment.id, SegmentType::Unknown(3));

    assert_eq!(unknown_segment.chunk.children_length, 4);

    let soundinfoitem = &unknown_segment.chunk.children[0];
    assert_eq!(soundinfoitem.id, SegmentType::SoundInfoItem);
    assert_eq!(soundinfoitem.chunk.children_length, 0);

    let controller_assignments = &unknown_segment.chunk.children[1];
    assert_eq!(
        controller_assignments.id,
        SegmentType::ControllerAssignments
    );
    assert_eq!(controller_assignments.chunk.children_length, 0);

    let encryption_item = &unknown_segment.chunk.children[2];
    assert_eq!(encryption_item.id, SegmentType::EncryptionItem);
    assert_eq!(encryption_item.chunk.children_length, 0);

    let unknown = &unknown_segment.chunk.children[3];
    assert_eq!(unknown.id, SegmentType::Unknown(4));
    assert_eq!(unknown.chunk.children_length, 0);
}

#[test]
fn test_container_parser() {
    Result::unwrap(glob::glob_with(
        "./tests/data/ni_container/*.*",
        glob::MatchOptions {
            case_sensitive: false,
            require_literal_separator: false,
            require_literal_leading_dot: false,
        },
    ))
    .for_each(|file| {
        let path = file.as_ref().unwrap();
        println!("\ntesting {:?}", &path);

        let buffer = std::fs::read(path).unwrap();
        let container = ni_file::ni_container::NIContainer::read(&buffer).expect("file to read");
        let container = container.data;

        assert_eq!(
            container.length,
            buffer.len() as u64,
            "container.length {:?}",
            path
        );
        assert_eq!(container.unknown_a, 1);
        assert_eq!(container.tag, ['h', 's', 'i', 'n']);
        assert_eq!(container.id, 1);
        assert_eq!(container.current_index, 1, "current_index in {:?}", path);
        assert_eq!(container.children_length, container.children.len() as u32);

        //assert_eq!(container.children[0].id, 101, "{:?}", path);

        // for child in container.children {
        // }
    });
}

// #[test]
// fn test_k5_4_demo() {
//     // checksums:
//     // u32le;4  0d881716e742958f4695d6978a38bca0
//     // u16le;8  17160d88958fe742d6974695bca08a38
//     // u8;16    1617880d8f9542e797d69546a0bc388a

//     let file = include_bytes!("./data/ni_container/kontakt-5.4-demo.nki");
//     let container = ni_file::ni_container::read(file).unwrap();
// }

// #[test]
// fn test_fm8_fm7() {
//     let file = include_bytes!("./data/ni_container/fm8--fm7.nfm8");
//     let container = ni_file::ni_container::read(file).unwrap();
// }

// #[test]
// fn test_massive_mexp() {
//     let file = include_bytes!("./data/ni_container/massive--mexp.nmsv");
//     let container = ni_file::ni_container::read(file).unwrap();
// }

// #[test]
// fn test_maschine_standard_2() {
//     let file = include_bytes!("./data/ni_container/maschine--standard-2.mxfx");
//     let container = ni_file::ni_container::read(file).unwrap();
// }

// // #[test]
// // fn test_guitar_rig_rammfire() {
// //     let file = include_bytes!("./data/ni_container/guitar-rig--rammfire.ngrr");
// //     let container = ni_file::ni_container::read(file).unwrap();
// // }

use ni_file::ni_segment::SegmentType;
