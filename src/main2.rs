use nom::{self, bytes, IResult};
use std::io;
use cb::Offset;

mod cb;
mod offset;
mod LZ77;

fn main() -> io::Result<()> {
    const FILE: &'static [u8] = include_bytes!("../examples/compressed-body");
    // const FILE: &'static [u8] = include_bytes!("../examples/header-without-filesize");
    let mut stack: Vec<u8> = include_bytes!("../examples/uncompressed-header").to_vec();


    println!("\nstack:");
    for byte in stack.clone() {
        print!("{:02X} ", byte);
    }
    println!("\n");

    println!("text:");
    for byte in stack {
        print!("{}", byte as char);
    }
    println!("\n");

    Ok(())
}

fn take_bytes(i: &[u8], l: usize) -> IResult<&[u8], &[u8]> {
    bytes::complete::take(l)(i)
}
