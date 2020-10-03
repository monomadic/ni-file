use std::{fs::File, io};
use io::Write;

mod cb;
mod offset;
mod deflate;
mod ni;
mod output;
mod structures;
mod fm8;
mod strings;
mod kontakt;

pub(crate) fn readfile(path: &str) -> Result<Vec<u8>, std::io::Error> {
    use std::io::prelude::*;

    let mut f = std::fs::File::open(path)?;
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer)?;

    Ok(buffer)
}

fn main() -> io::Result<()> {
    // const FILE: &'static [u8] = include_bytes!("../test-data/kontakt-5.4.1.189/Memory Bell Lowpass.nki");
    // const FILE: &'static [u8] = include_bytes!("../test-data/massive-1.5.5.22/test-se.nmsv");

    let args: Vec<String> = std::env::args().collect();

    if let Some(path) = args.get(1) {
        let file = readfile(&args[1])?;
        match ni::read(&file) {
            Ok(f) => output::print_segment(&f.1),
            Err(e) => println!("error: {:?}", e)
        }
    } else {
        panic!("no input file specified.");
    }
    

    // let (_, data) = ni::parse_data_segment(FILE).unwrap();

    // println!("{:?}", data);



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
