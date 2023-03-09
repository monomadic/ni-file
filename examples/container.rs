use std::convert::TryFrom;

use ni_file::{EncryptionItem, ItemID, NIContainer, NIFileType, RepositoryRoot};

#[allow(dead_code)]
fn setup_logger() {
    let _ = log::set_logger(&loggy::Loggy {
        prefix: "",
        show_time: false,
        show_thread: true,
    });
    log::set_max_level(log::LevelFilter::Debug); // Or whatever level you want.
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // uncomment this to use debug logging:
    // setup_logger();

    // lets read a Kontakt 7 file.
    let file = include_bytes!("../tests/data/files/kontakt-7/000-default.nki").as_slice();

    // make sure this is a valid NIContainer
    if NIFileType::detect(file) == NIFileType::NIContainer {
        // read the repository
        let repo = NIContainer::read(file)?;

        // parse the data fields of the base frame
        let root: RepositoryRoot = repo.root()?;

        // print the major version number
        println!(
            "NISound Version: {}.{}.{}",
            root.major_version(),
            root.minor_version(),
            root.patch_version(),
        );

        // iterate children
        println!("Children Found: {}", repo.children().len());
        for item in repo.children() {
            println!("Child Found: {:?}", item.data()?.header.item_id);

            for item in &item.children {
                println!(" Child Found: {:?}", item.data()?.header.item_id);
            }
        }

        // lets find an item frame of type EncryptionItem
        let e: EncryptionItem = repo
            .find(ItemID::EncryptionItem)
            .map(EncryptionItem::try_from)
            .unwrap()?;

        let preset = ni_file::Item::read(e.subtree.inner_data.as_slice())?;
        for item in &preset.children {
            println!("{:?}", item.data()?);
        }
    }

    Ok(())
}
