//
//  Decompress NCW files into PCM data.
//

use std::fs::File;

use color_eyre::eyre::Result;
use ni_file::ncw::NcwReader;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_BACKTRACE", "1");
    color_eyre::install()?;

    let Some(path) = std::env::args().nth(1) else {
        println!("usage: ni-ncw <FILE>");
        return Ok(());
    };

    let file = File::open(&path)?;
    let mut reader = NcwReader::read(file)?;

    let mut outfile = File::create("sample.wav")?;
    ni_file::ncw::write_wav(&mut reader, &mut outfile).unwrap();

    Ok(())
}
