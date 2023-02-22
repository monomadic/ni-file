use ni_file::{
    repository::{Item, ItemFrame, Repository, RepositoryRoot},
    NIFileType,
};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // lets read a Kontakt 7 file.
    let file = include_bytes!("../tests/data/files/kontakt-7/000-default.nki").as_slice();

    // make sure this is a valid repository
    if NIFileType::detect(file) == NIFileType::Repository {
        // read the repository
        let repo = Repository::read(file)?;

        // grab the root item
        let item: Item = repo.into();
        let frame: ItemFrame = item.frame_stack()?.pop()?;

        match frame {
            ItemFrame::RepositoryRoot(root) => {
                // print the major version number
                println!("version: {:?}", root.version());
                println!("magic: {}", root.magic);

                // println!("children: {}", repo.as_item().children().unwrap().len());
            }
            _ => (),
        }
    }

    Ok(())
}
