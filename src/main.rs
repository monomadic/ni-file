use std::io;

mod cb;
mod offset;
mod LZ77;
mod ni;

fn main() -> io::Result<()> {
    const FILE: &'static [u8] = include_bytes!("../examples/test-se.nmsv");
    // LZ77::parse(FILE);
    // LZ77::deflate(FILE, 1059);
    LZ77::deflate(FILE, 1040);
    Ok(())
}
