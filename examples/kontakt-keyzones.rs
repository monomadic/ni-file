// An example which prints all key zones in a Kontakt instrument.

use std::fs::File;

use color_eyre::eyre::{Report, Result};
use ni_file::NIFile;

pub fn main() -> Result<(), Report> {
    color_eyre::install()?;

    let Some(path) = std::env::args().nth(1) else {
        println!("usage: kontakt-info <FILE_GLOB>");
        return Ok(());
    };

    let file = File::open(&path)?;
    let ni = NIFile::read(file)?;

    // .. loop zone data

    Ok(())
}
