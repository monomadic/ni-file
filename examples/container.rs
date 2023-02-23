use ni_file::{repository::Repository, NIFileType};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // lets read a Kontakt 7 file.
    let file = include_bytes!("../tests/data/files/kontakt-7/000-default.nki").as_slice();

    // make sure this is a valid repository
    if NIFileType::detect(file) == NIFileType::Repository {
        // read the repository
        let mut repo = Repository::read(file)?;

        // print the major version number
        let root = repo.root()?;
        println!("version: {:?}", root.version());
        println!("magic: {}", root.magic);

        // iterate children
        println!("children found: {}", repo.children()?.len());
        for item in repo.children()? {
            println!("{:?}", item.frame_stack()?.frame()?)
        }
    }

    Ok(())
}
