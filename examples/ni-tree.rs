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
    std::env::set_var("RUST_BACKTRACE", "1");
    color_eyre::install()?;

    let Some(path) = std::env::args().nth(1) else {
        println!("usage: ni-tree <FILE_GLOB>");
        return Ok(());
    };

    let paths: Vec<std::path::PathBuf> = glob::glob(&path)
        .expect("glob error")
        .filter_map(|path| path.ok())
        .filter(|path| path.file_name().unwrap() != ".DS_Store")
        .collect();

    // repository containers (used in most instruments)
    for path in paths {
        println!("\n{}:", path.as_os_str().to_str().unwrap());

        let item = Item::read(std::fs::File::open(path)?)?;
        println!("{:?}", item.data()?.header.item_id);

        print_item_ids(&item, 1)?;
    }

    Ok(())
}
