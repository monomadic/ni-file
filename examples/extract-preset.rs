use ni_file::{NIContainer, NIFileType};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // lets read a Kontakt 7 file.
    let file = include_bytes!("../tests/data/files/kontakt-7/000-default.nki").as_slice();

    // make sure this is a valid NIContainer
    if NIFileType::detect(file) == NIFileType::NIContainer {
        // read the repository
        let repo = NIContainer::read(file)?;
        let preset = repo.preset()?;
    }

    Ok(())
}
