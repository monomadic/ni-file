use nom::{IResult, bytes, number::complete::le_i32};
use crate::cb::Offset;
use crate::offset;

// pub fn parse(file: &[u8]) {
//     // read filesize chunk
//     let (r, header) = read_file_header(file).unwrap();

//     // check filesize is valid
//     if header.filesize != file.len() as i32 {
//         panic!("error: file header does not match up with file size.\n");
//     } else {
//         println!("file size: {} bytes.\n", header.filesize);
//     }

//     // parse
//     parse_chunks(r);
// }

pub(crate) fn deflate(i: &[u8], offset: usize) -> IResult<&[u8], Vec<u8>> {
    // let mut rem = &i.clone()[offset..i.len()];
    // let mut stack: Vec<u8> = Vec::new();

    let (stack, mut rem ) = i.split_at(offset);
    let mut stack = stack.to_vec();

    loop {
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
                    // break;
                }
                Offset::Literal { length } => {
                    let (r, bytes) = take_bytes(rem, length).unwrap();
                    rem = r;

                    print!("LITERAL BUFFER PUSH: ");
                    for byte in bytes.clone() {
                        print!("{:02X} ", byte);
                    }
                    println!("\n");

                    stack.append(&mut bytes.to_vec());
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