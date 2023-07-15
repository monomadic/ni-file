///
/// fastlz decompression implementation in pure rust
///
use crate::{read_bytes::ReadBytesExt, NIFileError};

/// deflate with size check
pub fn deflate_checked(
    compressed_input: &[u8],
    decompressed_len: usize,
) -> Result<Vec<u8>, NIFileError> {
    let buf = deflate(compressed_input)?;

    assert_eq!(buf.len(), decompressed_len);
    Ok(buf.to_vec())
}

/// deflate as a stream
pub fn deflate<R: ReadBytesExt>(mut reader: R) -> Result<Vec<u8>, NIFileError> {
    let mut dictionary = Vec::new();

    loop {
        match get_control_bytes(&mut reader) {
            Ok(offset) => match offset {
                Offset::Dictionary { length, offset } => {
                    let mut dict = fetch_offset(&dictionary, length, offset);
                    dictionary.append(&mut dict);
                }
                Offset::Literal { length } => match reader.read_bytes(length) {
                    Ok(bytes) => {
                        dictionary.append(&mut bytes.to_vec());
                    }
                    Err(_) => {
                        // panic!("error: cannot take any more literal bytes, reached end of compressed buffer.");
                        break;
                    }
                },
            },
            Err(_) => {
                break;
            }
        }
    }

    Ok(dictionary)
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

#[derive(Debug, PartialEq)]
pub(crate) enum Offset {
    Literal { length: usize },
    Dictionary { length: usize, offset: usize },
}

/// Control bytes are used by the compression algorithm to determine what kind of data is in the next chunk.
pub(crate) fn get_control_bytes<R: ReadBytesExt>(mut reader: R) -> Result<Offset, NIFileError> {
    let cb = reader.read_u8()?;
    let q = q_mask(cb) as usize;
    let cb_mask = cb_mask(cb) as usize;

    Ok(match cb_mask {
        1 => Offset::Literal { length: 1 + q },
        3..=8 => {
            let r = reader.read_u8()?;
            Offset::Dictionary {
                length: cb_mask,
                offset: ((q << 8) + r as usize + 1),
            }
        }

        9 => {
            let r = reader.read_u8()?;
            let s = reader.read_u8()?;

            Offset::Dictionary {
                length: 9 + r as usize,
                offset: ((q << 8) + s as usize + 1),
            }
        }
        _ => panic!(),
    })
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

    if i | 0b1001_1111 == 0b1001_1111 {
        return 6;
    }

    if i | 0b1011_1111 == 0b1011_1111 {
        return 7;
    }

    if i | 0b1101_1111 == 0b1101_1111 {
        return 8;
    }

    if i | 0b1111_1111 == 0b1111_1111 {
        return 9;
    }

    panic!("unknown control byte. [{:08b}:{:02X}]", i, i);
}

/// bitwise mask to determine 'Q'
fn q_mask(i: u8) -> u8 {
    i & 0b0001_1111
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cb_mask() {
        assert_eq!(cb_mask(0b00000001), 1);
        assert_eq!(cb_mask(0b00100001), 3);
        assert_eq!(cb_mask(0b01000001), 4);
        assert_eq!(cb_mask(0b01100001), 5);
        assert_eq!(cb_mask(0b10000001), 6);
        assert_eq!(cb_mask(0b10100001), 7);
        assert_eq!(cb_mask(0b11000101), 8);
        assert_eq!(cb_mask(0b11100001), 9);
    }

    #[test]
    fn test_q_mask() {
        assert_eq!(q_mask(0b11100001), 1);
        assert_eq!(q_mask(0b11100010), 2);
        assert_eq!(q_mask(0b00000011), 3);
    }

    #[test]
    fn test_get_control_bytes() -> Result<(), NIFileError> {
        use Offset::*;

        assert_eq!(get_control_bytes([0x02].as_slice())?, Literal { length: 3 });

        assert_eq!(
            get_control_bytes([0x20, 0x0E].as_slice())?,
            Dictionary {
                length: 3,
                offset: 15
            }
        );

        assert_eq!(
            get_control_bytes([0x60, 0x00].as_slice())?,
            Dictionary {
                length: 5,
                offset: 1
            }
        );

        Ok(())
    }

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
        let input = include_bytes!("../tests/data/nisound/fastlz/kontakt-4/001-garbo2.compressed");
        assert_eq!(
            deflate(input.as_slice()).unwrap(),
            include_bytes!("../tests/data/nisound/fastlz/kontakt-4/001-garbo2.decompressed")
        )
    }

    #[test]
    fn test_decompress() {
        let compressed_input =
            include_bytes!("../tests/data/nisound/fastlz/kontakt-4/001-garbo2.compressed");
        let expected_output =
            include_bytes!("../tests/data/nisound/fastlz/kontakt-4/001-garbo2.decompressed");

        let decompressed_output =
            deflate_checked(compressed_input, expected_output.len()).expect("decompression failed");

        assert_eq!(expected_output.to_vec(), decompressed_output);
    }
}
