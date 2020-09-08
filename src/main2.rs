use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let mut f = File::open("examples/compressed-portion-only")?;
    // let mut buffer = [0; 1];
    // f.read_exact(&mut buffer)?;

    // for byte in buffer.iter() {
    //     print!("{:2} ", byte.clone() as char);
    // }

    // println!("");

    // for byte in buffer.iter() {
    //     print!("{:X} ", byte);
    // }

    // println!("");

    let mut buffer = [0; 1];
    f.read_exact(&mut buffer)?;

    for byte in buffer.iter() {
        println!("CONTROL BYTE: {:b} {:X}", byte, byte);

        // match byte | 0b1111_1111 {
        //     0b0011_1111 {

        //     }
        //     _ => ()
        // }

        // 0b0000_0010
        // 0b0001_1111

        // 0b0001_1111

        

        if byte | 0b0001_1111 == 0b0001_1111 {
            println!("0x000XXXXX LITERAL");
        }



        if byte | 0b0011_1111 == 0b1111_1111 {
            // 0x001XXXXX

            println!("0x001XXXXX TOKEN TYPE: DICT");
            println!("LENGTH: 3");
            println!("OFFSET: 0");
        }

    }
    // println!("The bytes: {:?}", &buffer);
    Ok(())
}

fn bit_at(input: &u8, pos: u8) -> bool {
    if pos < 32 {
        input & (1 << pos) != 0
    } else {
        panic!()
    }
}