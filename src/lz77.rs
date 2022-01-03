use crate::container::SegmentType;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Read;
use std::io::Write;

pub fn deflate(mut buffer: &[u8]) -> Result<&[u8], Box<dyn std::error::Error>> {
    // let mut dictionary = vec![];

    loop {
        // read the next control byte
        let cb = buffer.read_u8().unwrap();

        let q = q_mask(cb) as usize;
        let cb_mask = cb_mask(cb) as usize;
        match cb_mask {
            1 => {
                // just 1 literal byte
                // grab 1 + q bytes from the buffer
                // 1 + q
            }
            3..=8 => {
                // dictionary lookup
                let r = buffer.read_u8().unwrap();
                let offset = (q << 8) + r as usize + 1;
                // length cb_mask
            }
            9 => {
                let r = buffer.read_u8().unwrap();
                let s = buffer.read_u8().unwrap();

                let length = 9 + r as usize;
                let offset = (q << 8) + s as usize + 1;
            }
            _ => panic!("invalid control byte during deflate"),
        }
    }
}

// #[test]
// fn test_get_control_bytes() {
//     assert_eq!(
//         &format!("{:?}", get_control_bytes(&[0x02]).unwrap()),
//         "([], Literal { length: 3 })"
//     );

//     assert_eq!(
//         &format!("{:?}", get_control_bytes(&[0x20, 0x0E]).unwrap()),
//         "([], Dictionary { length: 3, offset: 15 })"
//     );

//     assert_eq!(
//         &format!("{:?}", get_control_bytes(&[0x60, 0x00]).unwrap()),
//         "([], Dictionary { length: 5, offset: 1 })"
//     );
// }

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

/// bitwise mask to determine 'Q'
fn q_mask(i: u8) -> u8 {
    i & 0b0001_1111
}

#[test]
fn test_q_mask() {
    assert_eq!(q_mask(0b11100001), 1);
    assert_eq!(q_mask(0b11100010), 2);
    assert_eq!(q_mask(0b00000011), 3);
}
