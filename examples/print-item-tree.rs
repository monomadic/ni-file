// This example shows the basic hierarchical structure of an NISound container

use std::error::Error;

use ni_file::nisound::Item;

fn print_item_ids(item: &Item, indent: usize) -> Result<(), Box<dyn Error>> {
    for item in &item.children {
        println!(
            "{:>width$}{:?}",
            " ",
            item.data()?.header.item_id,
            width = indent
        );
        print_item_ids(&item, indent + 1)?;
    }

    Ok(())
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let paths: Vec<std::path::PathBuf> = glob::glob("tests/data/files/**/*.*")?
        .filter_map(|path| path.ok())
        .filter(|path| path.file_name().unwrap() != ".DS_Store")
        .collect();

    for path in paths {
        println!("\n{}:", path.as_os_str().to_str().unwrap());

        let item = Item::read(std::fs::File::open(path)?)?;
        println!("{:?}", item.data()?.header.item_id);

        print_item_ids(&item, 1)?;
    }

    Ok(())
}
