use binread::{io::Cursor, prelude::*};
use ni_file::ni_object::NIData;

#[test]
fn test_kontakt5_repository_root() {
    let file = include_bytes!("../data/items/kontakt-5-repository-root");

    let data: NIData = ni_file::ni_object::read_data(file).unwrap();

    panic!("{:?}", data);
}
