use ni_file::prelude::*;

#[test]
fn test_kontakt() -> std::result::Result<(), NIFileError> {
    let file = include_bytes!("./data/ni_container/kontakt-5.4-demo.nki");
    let container = ni_file::raw_repository::Repository::read(file)?;

    assert_eq!(
        container.number_of_children as usize,
        container.children.iter().count()
    );

    Ok(())
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
