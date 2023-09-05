//
//  Decompress NCW files into PCM data.
//

use color_eyre::eyre::Result;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_BACKTRACE", "1");
    color_eyre::install()?;

    let Some(path) = std::env::args().nth(1) else {
        println!("usage: ni-extract <FILE>");
        return Ok(());
    };

    //let file = std::fs::read(&path)?;

    ni_file::ncw::decode(&path).unwrap();

    Ok(())
}
