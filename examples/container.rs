use ni_file::{repository::Item, NIFileType};

pub fn main() {
    // lets read a Kontakt 7 file.
    let file = include_bytes!("../tests/data/files/kontakt-7/000-default.nki");

    // make sure this is a valid repository
    if NIFileType::detect(file) == NIFileType::Repository {
        Item::read(file.as_slice()).unwrap();
    }
}
