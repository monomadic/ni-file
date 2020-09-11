use nom::{IResult, bytes, number::complete::le_i32};
use crate::cb::Offset;
use crate::offset;

pub(crate) fn deflate(i: &[u8], offset: usize) -> IResult<&[u8], Vec<u8>> {
    let (stack, mut rem ) = i.split_at(offset);
    let mut stack = stack.to_vec();

    loop {
        println!("file offset :{} {:?}", i.len() - rem.len(), &rem[0..7]);

        if rem[0..7] == [0x0D, 0x00, 0x00, 0x0D, 0x62, 0x65, 0x85] {
            // be. (block end) tag found.
            break;
        }

        if let Ok((r, o)) = crate::cb::get_control_bytes(rem) {
            rem = r;
            println!("{:?}", o);

            match o {
                Offset::Dictionary { length, offset } => {
                    let mut dict = offset::fetch_offset(&stack, length, offset);
                    print!("DICT BUFFER PUSH: ");
                    for byte in dict.clone() {
                        print!("{:02X} ", byte);
                    }
                    println!("\n");

                    stack.append(&mut dict);
                }
                Offset::Literal { length } => {
                    // let (r, bytes) = take_bytes(rem, length)?;
                    if let Ok((r, bytes)) = take_bytes(rem, length) {
                        rem = r;

                        print!("LITERAL BUFFER PUSH: ");
                        for byte in bytes.clone() {
                            print!("{:02X} ", byte);
                        }
                        println!("\n");
    
                        stack.append(&mut bytes.to_vec());
                    } else {
                        break;
                    }
                }
            }
        } else {
            break;
        }
    }

    Ok((rem, stack))
}

#[test]
fn test_deflate() {
    // assert_eq!(deflate(b"").unwrap().1, []);
    // assert_eq!(deflate(b"").unwrap().1, []);
}

struct FileHeader {
    filesize: i32,
    unknown1: i32, // 0, optional
    unknown2: i32, // 1. optional
}

fn read_file_header(i: &[u8]) -> IResult<&[u8], FileHeader> {
    nom::combinator::map(
        nom::sequence::tuple((le_i32, le_i32, le_i32)), |(filesize, unknown1, unknown2)| FileHeader {
            filesize, unknown1, unknown2
        }
    )(i)
}

fn take_bytes(i: &[u8], l: usize) -> IResult<&[u8], &[u8]> {
    bytes::complete::take(l)(i)
}