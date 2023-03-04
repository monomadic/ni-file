use ni_file::Item;
use std::error::Error;

fn print_item_ids(item: &Item, indent: usize) -> Result<(), Box<dyn Error>> {
    for item in &item.children {
        println!(
            "{:>width$}{:?}",
            " ",
            item.frame()?.header.item_id,
            width = indent
        );
        print_item_ids(&item, indent + 1)?;
    }

    Ok(())
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = include_bytes!("../tests/data/files/kontakt-7/000-default.nki").as_slice();

    let item = Item::read(file)?;
    println!("{:?}", item.frame()?.header.item_id);

    print_item_ids(&item, 1)?;

    Ok(())
}
