// use ni_file::ni_object::NIObject;
use ni_file::prelude::*;

#[test]
fn via_raw_data_test() -> Result<()> {
    Ok(())
}

// #[test]
// fn test_kontakt_5() {
//     let file = include_bytes!("./data/ni_container/kontakt-5.4-demo.nki");
//     let container = ni_file::raw_container::Repository::read(file);
//
//     let container = container.expect("file to read");
//
//     let container: NIObject = container.into();
//     panic!("{:?}", container);
//
//     // assert_eq!(format!("{:?}", container), "".to_string());

//
// #[test]
// fn test_fm8() {
//     let file = include_bytes!("./data/ni_container/fm8--fm7.nfm8");
//     let container = ni_file::raw_container::Repository::read(file);
//     let container = container.expect("file to read");
//
//     panic!("{:?}", container.data());
// }
