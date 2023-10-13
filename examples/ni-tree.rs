// This example shows the basic hierarchical structure of an NISound container

use std::{error::Error, path::PathBuf};

use ni_file::nis::ItemContainer;

fn get_files(path: &str) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    Ok(glob::glob(path)?
        .filter_map(|path| path.ok())
        .filter(|path| path.file_name().unwrap() != ".DS_Store")
        .collect())
}

fn print_item_ids(item: &ItemContainer, indent: usize) -> Result<(), Box<dyn Error>> {
    for item in &item.children {
        print!(
            "{:>width$}{:?}",
            " ",
            item.data.header.item_type(),
            width = indent
        );

        if let Some(inner) = &item.data.inner {
            print!(", {:?}", inner.header.item_type());
            // two levels down?
            if let Some(inner) = &inner.inner {
                print!(", {:?}", inner.header.item_type());
            }
        }

        print!("\n");

        print_item_ids(&item, indent + 1)?;
    }

    Ok(())
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_BACKTRACE", "1");
    color_eyre::install()?;

    let Some(path) = std::env::args().nth(1) else {
        println!("usage: ni-tree <FILE_GLOB>");
        return Ok(());
    };

    let paths = get_files(&path)?;

    // repository containers (used in most instruments)
    for path in paths {
        let file = std::fs::File::open(path)?;
        let repository = ni_file::Repository::read(&file)?;
        let item = repository.item();

        if let Some(Ok(root)) = repository.find_repository_root() {
            println!("NISound {}\n", root.nisound_version);
        }

        println!("{:?}", item.data.header.item_type());

        print_item_ids(&item, 1)?;

        println!("\n");
    }

    Ok(())
}
