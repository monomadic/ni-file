use ni_file::{repository::Repository, NIFileType};

pub fn main() {
    // lets read a Kontakt 7 file.
    let file = include_bytes!("../tests/data/files/kontakt-7/000-default.nki").as_slice();

    // make sure this is a valid repository
    if NIFileType::detect(file) == NIFileType::Repository {
        // read the repository
        let repo = Repository::read(file).unwrap();
        // grab the root item
        let root = repo.root_item().unwrap();
        // print the major version number
        println!("major version: {}", root.major_version());
    }
}
