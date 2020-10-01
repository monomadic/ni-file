use std::{fs::File, io};
use io::Write;

mod cb;
mod offset;
mod deflate;
mod ni;
mod output;
mod structures;

fn main() -> io::Result<()> {
    // const FILE: &'static [u8] = include_bytes!("../test-data/kontakt-5.4.1.189/Memory Bell Lowpass.nki");
    const FILE: &'static [u8] = include_bytes!("../test-data/massive-1.5.5.22/test-se.nmsv");

    // let (_, data) = ni::parse_data_segment(FILE).unwrap();

    // println!("{:?}", data);

    match ni::read(FILE) {
        Ok(f) => output::print_segment(f.1),
        Err(e) => println!("error: {:?}", e)
    }

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
