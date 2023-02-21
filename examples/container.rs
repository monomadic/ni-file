use ni_file::{repository::Repository, NIFileType};

pub fn main() {
    // lets read a Kontakt 7 file.
    let file = include_bytes!("../tests/data/files/kontakt-7/000-default.nki");

    // make sure this is a valid repository
    if NIFileType::detect(file) == NIFileType::Repository {
        let repo = Repository::read(file.as_slice()).unwrap();
        println!(
            "major version: {}",
            repo.root_item().unwrap().major_version()
        );
    }
}
