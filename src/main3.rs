use nom::{self, bits, bytes, IResult};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use cb::Offset;

mod cb;

fn decompress_lz77(i: &[u8]) -> IResult<&[u8], &[u8]> {
    // let (r, filesize) = header(file)?;
    take_filesize_header(i)
}

fn take_filesize_header(i: &[u8]) -> IResult<&[u8], &[u8]> {
    bytes::complete::take(2u32)(i)
}

fn take_3_bits(input: &[u8]) -> IResult<&[u8], u64> {
    bits(bits::complete::take::<_, _, _, (_, _)>(3usize))(input)
}

fn take_5_bits(input: &[u8]) -> IResult<&[u8], u64> {
    bits(bits::complete::take::<_, _, _, (_, _)>(5usize))(input)
}







// #[test]
// fn test_get_offset() {
//     assert_eq!(
//         get_offset(&[0b00000001]).unwrap().0,
//         &[0]
//     );
// }

// fn parse(i: &[u8]) -> IResult<&[u8], Offset> {
//     many1(get_control_bytes)
// }

fn main() -> io::Result<()> {
    // let mut f = File::open("examples/compressed-portion-only")?;
    // let mut buffer = String::new();
    // f.read_to_string(&mut buffer)?;

    // const buffer: &'static [u8] = include_bytes!("../examples/compressed-portion-only");
    
    
    // &fs::read("address.txt")?

    // let (r, cb) = take_control_byte(buffer).unwrap();
    // println!("{:X}", cb[0]);

    // let (r, cb) = take_control_byte(&[0b00100001]).unwrap();
    // let (r, cb) = take_control_byte(&[0b00100000]).unwrap();

    // println!("{:?}",
    //     bits(bits::complete::take::<_, _, _, (_, _)>(1usize))(&[0b00100001])
    // );

    // let rem = String::from("TESTSTRING1 PREFIXàSUF ");
    // let rem = rem.as_bytes();

    // let rem = &[0x0B, 0x54];
    const FILE: &'static [u8] = include_bytes!("../examples/compressed-body");
    // const FILE: &'static [u8] = include_bytes!("../examples/simple");

    // let mut rem: &[u8] = buffer.clone();
    let mut stack: Vec<u8> = include_bytes!("../examples/uncompressed-header").to_vec();
    // let mut stack: Vec<u8> = vec![0,0,0,0,0];

    let mut rem = FILE.clone();

    loop {
        // println!("text:");
        // for byte in stack.clone() {
        //     print!("{}", byte as char);
        // }
        // println!("\n");

        if let Ok((r, o)) = cb::get_control_bytes(rem) {
            rem = r;
            println!("{:?}", o);

            match o {
                Offset::Dictionary { length, offset } => {
                    let mut dict = fetch_offset(&stack, offset, length);
                    println!("FOUND OFFSET: {:?}", dict);
                    stack.append(&mut dict);
                    // break;
                }
                Offset::Literal { length } => {
                    let (r, bytes) = take_bytes(rem, length).unwrap();
                    rem = r;

                    stack.append(&mut bytes.to_vec());
                }
            }
        } else {
            break;
        }

        // break;
    }

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
    
    

    // let (r, o) = get_control_bytes(rem).unwrap();
    // rem = r;
    // println!("{:?}", o);

    // match o {
    //     Offset::Dictionary { length, offset } => {
            
    //     }
    //     Offset::Literal { length } => {
    //         let (r, bytes) = take_bytes(rem, length).unwrap();
    //         rem = r;

    //         for byte in bytes {
    //             print!("{:X} ", byte);
    //         }
    //     }
    // }

    // println!("{:?}", get_offset(&[0b00000010, 0b11111001, 0b00010101]));
    // println!("{:?}", get_offset(&[0b01100000, 0b00000000, 0b00000000]));

    Ok(())
}

fn take_bytes(i: &[u8], l: usize) -> IResult<&[u8], &[u8]> {
    bytes::complete::take(l)(i)
}

// fn get_chunk(i: &[u8], buffer: &mut Vec<u8>) -> IResult<&[u8], &[u8]> {
//     let (r, o) = get_control_bytes(i)?;
//     // get_control_bytes(&i)
//     //     .map(|(r, o)| {
//     //         (r, r)
//     //     })

//     match o {
//         Offset::Dictionary { length, offset } => {
//             panic!();
//         }
//         Offset::Literal { length } => {
//             let (r, bytes) = take_bytes(r, length)?;

//             let b: &mut Vec<u8> = &mut bytes.clone().to_vec();

//             // buffer.append(bytes);
//             // for byte in bytes {
//             //     print!("{:X} ", byte); 
//             // }

//             return Ok((r, bytes));
//         }
//     }

//     // Ok((&[], &[]))
// }
