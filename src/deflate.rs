///
/// fastlz implementation in pure rust
///
use crate::cb::Offset;
use nom::{bytes, IResult};

// TODO: remove `nom` dependency
// TODO: rewrite into reader trait form

pub fn deflate(i: &[u8], offset: usize) -> IResult<&[u8], Vec<u8>> {
    // anything before the offset becomes the dictionary
    let (_dictionary, mut rem) = i.split_at(offset);
    let mut dictionary = vec![];

    loop {
        if let Ok((r, o)) = crate::cb::get_control_bytes(rem) {
            rem = r;

            match o {
                Offset::Dictionary { length, offset } => {
                    let mut dict = fetch_offset(&dictionary, length, offset);
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

fn take_bytes(i: &[u8], l: usize) -> IResult<&[u8], &[u8]> {
    bytes::complete::take(l)(i)
}

pub(crate) fn fetch_offset(buffer: &Vec<u8>, length: usize, offset: usize) -> Vec<u8> {
    if offset > buffer.len() {
        panic!("Cannot deflate: offset seek is larger than dictionary.");
    }

    (0..length)
        .map(|index| {
            let start_pos = buffer.len() - offset;
            let offset_pos = start_pos + index;

            if length > offset {
                // let circular_pos = offset_pos - offset;
                // panic!("attempt {:?}",  (buffer.len(), offset, index));
                let circular_pos = start_pos + (index % offset);
                if circular_pos > buffer.len() {
                    panic!("attempt {:?}", (circular_pos, offset, buffer.len()));
                }
                buffer[circular_pos]
            } else {
                buffer[offset_pos]
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_offset() {
        assert_eq!(
            fetch_offset(&vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07], 3, 7),
            vec![0x01, 0x02, 0x03]
        );

        assert_eq!(
            fetch_offset(&vec![0x01, 0x02, 0x03, 0xF4, 0x15, 0x06], 1, 5),
            vec![0x02]
        );

        assert_eq!(
            fetch_offset(&vec![0x00, 0x01, 0x00, 0x00, 0x00], 16, 4),
            vec![
                0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00,
                0x00, 0x00
            ]
        );

        assert_eq!(
            fetch_offset(&vec![0x01, 0x02, 0xF4, 0x08, 0x00], 3, 1),
            vec![0x00, 0x00, 0x00]
        );
    }

    #[test]
    fn test_deflate() {
        assert_eq!(
            deflate(
                include_bytes!("../tests/data/nisound/fastlz/kontakt-4/001-garbo2.compressed"),
                0
            )
            .unwrap()
            .1,
            include_bytes!("../tests/data/nisound/fastlz/kontakt-4/001-garbo2.decompressed")
        )
    }
}
