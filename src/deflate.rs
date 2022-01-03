use crate::cb::Offset;
use crate::offset;
use nom::{bytes, IResult};

pub(crate) fn deflate(i: &[u8], offset: usize) -> IResult<&[u8], Vec<u8>> {
    // anything before the offset becomes the dictionary
    let (_dictionary, mut rem) = i.split_at(offset);
    // let mut dictionary = dictionary.to_vec();
    let mut dictionary = vec![];

    loop {
        if let Ok((r, o)) = crate::cb::get_control_bytes(rem) {
            rem = r;

            match o {
                Offset::Dictionary { length, offset } => {
                    let mut dict = offset::fetch_offset(&dictionary, length, offset);
                    dictionary.append(&mut dict);
                }
                Offset::Literal { length } => {
                    if let Ok((r, bytes)) = take_bytes(rem, length) {
                        rem = r;
                        dictionary.append(&mut bytes.to_vec());
                    } else {
                        error!("error: cannot take any more literal bytes, reached end of compressed buffer.");
                        break;
                    }
                }
            }
        } else {
            break;
        }
    }

    Ok((rem, dictionary))
}

#[test]
fn test_deflate() {
    // assert_eq!(deflate(b"").unwrap().1, []);
    // assert_eq!(deflate(b"").unwrap().1, []);
}

fn take_bytes(i: &[u8], l: usize) -> IResult<&[u8], &[u8]> {
    bytes::complete::take(l)(i)
}
