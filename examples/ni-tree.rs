// This example shows the basic hierarchical structure of an NISound container

use std::error::Error;

use ni_file::nisound::Item;

fn print_item_ids(item: &Item, indent: usize) -> Result<(), Box<dyn Error>> {
    for item in &item.children {
        print!(
            "{:>width$}{:?}",
            "  ",
            item.data()?.header.item_id,
            width = indent
        );

        if let Some(inner) = item.data()?.inner() {
            print!(", {:?}", inner.header.item_id);
            // two levels down?
            if let Some(inner) = inner.inner() {
                print!(", {:?}", inner.header.item_id);
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

    let paths: Vec<std::path::PathBuf> = wax::Glob::new(&path)
        .unwrap()
        .walk(".")
        .flatten()
        .map(|entry| entry.into_path())
        .filter(|entry| entry.is_file())
        .filter(|path| path.file_name().unwrap() != ".DS_Store")
        .collect();

    // let paths: Vec<std::path::PathBuf> = glob::glob(&path)?
    //     .filter_map(|path| path.ok())
    //     .filter(|path| path.file_name().unwrap() != ".DS_Store")
    //     .collect();

    // repository containers (used in most instruments)
    for path in paths {
        println!("\n{}:", path.as_os_str().to_str().unwrap());

        let item = Item::read(std::fs::File::open(path)?)?;
        println!("{:?}", item.data()?.header.item_id);

        print_item_ids(&item, 1)?;
    }

    Ok(())
}
