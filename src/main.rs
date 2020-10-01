use std::{fs::File, io};
use io::Write;

mod cb;
mod offset;
mod deflate;
mod ni;

fn main() -> io::Result<()> {
    const FILE: &'static [u8] = include_bytes!("../examples/segments/data-segment-5.dsin");

    let (_, data) = ni::parse_data_segment(FILE).unwrap();

    // println!("{:?}", data);

    // match ni::read(FILE) {
    //     Ok(f) => println!("done\n{:?}", f.1),
    //     Err(e) => println!("error: {:?}", e)
    // }

    // match deflate::deflate(FILE, 1) {
    //     Ok((_, content)) => {
    //         let mut buffer = File::create("massive.deflate")?;
    //         buffer.write_all(&content)?;
    //         println!("done!");
    //     },
    //     Err(e) => println!("error: {:?}", e)
    // }

    Ok(())
}
