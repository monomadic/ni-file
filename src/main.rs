use nom::{self, bits, bytes, IResult};
use std::fs::File;
use std::io;
use std::io::prelude::*;

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

fn take_control_byte(i: &[u8]) -> IResult<&[u8], &[u8]> {
    let (r, cb) = bytes::complete::take(1usize)(i)?;

    // let (cb_r, ty) = bits::bits(bits::complete::take(3_usize)(cb)?);
    // let cb_type_mask = nom::bits::bits(nom::bits::complete::take::<_, _, _, (_, _)>(3_usize))(cb).unwrap();

    println!("matching: {:08b}", cb[0]);

    let byte = cb[0];

    if byte | 0b0001_1111 == 0b0001_1111 {
        println!("0x000XXXXX LITERAL");
    }

    let (cb_r, cb_type_mask) = take_3_bits(&cb)?;
    print!("[{:03b}]", cb_type_mask);

    let (r, cb_q) = take_5_bits(cb_r)?;
    print!("[{:05b}]", cb_q);

    Ok((r, cb))
}

fn cb_mask(i: u8) -> u8 {
    if i | 0b0001_1111 == 0b0001_1111 {
        return 1;
    }

    if i | 0b0011_1111 == 0b0011_1111 {
        return 3;
    }

    if i | 0b0101_1111 == 0b0101_1111 {
        return 4;
    }

    if i | 0b0111_1111 == 0b0111_1111 {
        return 5;
    }

    0
}

#[test]
fn test_cb_mask() {
    assert_eq!(cb_mask(0b00000001), 1);
    assert_eq!(cb_mask(0b00100001), 3);
    assert_eq!(cb_mask(0b01000001), 4);
    assert_eq!(cb_mask(0b01100001), 5);
    assert_eq!(cb_mask(0b10000001), 6);
    assert_eq!(cb_mask(0b10100001), 7);
    assert_eq!(cb_mask(0b11000001), 8);
    assert_eq!(cb_mask(0b11100001), 9);
}

fn q_mask(i: u8) -> u8 {
    i & 0b0001_1111
}

#[test]
fn test_q_mask() {
    assert_eq!(q_mask(0b11100001), 1);
    assert_eq!(q_mask(0b11100010), 2);
    assert_eq!(q_mask(0b00000011), 3);
}

#[derive(Debug)]
enum Offset {
    Literal { length: u32 },
    Dictionary { length: u32, offset: u32 },
}

fn get_offset(i: &[u8]) -> IResult<&[u8], Offset> {
    let (rem, cb) = bytes::complete::take(1u8)(i)?;

    println!("\ncb {:b} {:X}", cb[0], cb[0]);

    let q = q_mask(cb[0]) as u32;
    let cb_mask = cb_mask(cb[0]) as u32;

    // println!("cb_mask {:?} {:?}", cb[0], cb_mask(cb[0]));

    Ok(match cb_mask {
        1 => (rem, Offset::Literal { length: 1 + q }),
        3..=8 => {
            let (rem, r) = bytes::complete::take(1u8)(rem)?;
            (
                rem,
                Offset::Dictionary {
                    length: cb_mask,
                    offset: ((q << 8) + r[0] as u32 + 1),
                },
            )
        }
        _ => panic!(),
    })
}
// #[test]
// fn test_get_offset() {
//     assert_eq!(
//         get_offset(&[0b00000001]).unwrap().0,
//         &[0]
//     );
// }

fn main() -> io::Result<()> {
    // let mut f = File::open("examples/compressed-portion-only")?;
    // let mut buffer = String::new();
    // f.read_to_string(&mut buffer)?;

    const buffer: &'static [u8] = include_bytes!("../examples/compressed-portion-only");
    // &fs::read("address.txt")?

    // let (r, cb) = take_control_byte(buffer).unwrap();
    // println!("{:X}", cb[0]);

    // let (r, cb) = take_control_byte(&[0b00100001]).unwrap();
    // let (r, cb) = take_control_byte(&[0b00100000]).unwrap();

    // println!("{:?}",
    //     bits(bits::complete::take::<_, _, _, (_, _)>(1usize))(&[0b00100001])
    // );

    let mut rem: &[u8] = &[];

    let (r, o) = get_offset(buffer).unwrap();
    rem = r;
    println!("{:?}", o);

    match o {
        Offset::Dictionary { length, offset } => {
            
        }
        Offset::Literal { length } => {
            let (r, bytes) = take_bytes(rem, length).unwrap();
            rem = r;

            for byte in bytes {
                print!("{:X} ", byte);
            }
        }
    }
    

    let (r, o) = get_offset(rem).unwrap();
    rem = r;
    println!("{:?}", o);

    match o {
        Offset::Dictionary { length, offset } => {
            
        }
        Offset::Literal { length } => {
            let (r, bytes) = take_bytes(rem, length).unwrap();
            rem = r;

            for byte in bytes {
                print!("{:X} ", byte);
            }
        }
    }

    // println!("{:?}", get_offset(&[0b00000010, 0b11111001, 0b00010101]));
    // println!("{:?}", get_offset(&[0b01100000, 0b00000000, 0b00000000]));

    Ok(())
}

fn take_bytes(i: &[u8], l: u32) -> IResult<&[u8], &[u8]> {
    bytes::complete::take(l)(i)
}
