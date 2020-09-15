use std::{fs::File, io};
use io::Write;

mod cb;
mod offset;
mod deflate;
mod ni;

fn main() -> io::Result<()> {
    const FILE: &'static [u8] = include_bytes!("../examples/filterbass.nki");
    // LZ77::parse(FILE);

    match deflate::deflate(FILE, 517) {
        Ok((_, content)) => {
            let mut buffer = File::create("filterbass")?;
            buffer.write_all(&content)?;
            println!("done!");
        },
        Err(e) => println!("error: {:?}", e)
    }
    Ok(())
}
