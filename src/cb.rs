use nom::{bytes, IResult};

/// control byte

#[derive(Debug)]
pub(crate) enum Offset {
    Literal { length: usize },
    Dictionary { length: usize, offset: usize },
}

pub(crate) fn get_control_bytes(i: &[u8]) -> IResult<&[u8], Offset> {
    let (rem, cb) = bytes::complete::take(1u8)(i)?;

    // println!("CB: [{:08b}:{:02X}] ", cb[0], cb[0]);

    let q = q_mask(cb[0]) as usize;
    let cb_mask = cb_mask(cb[0]) as usize;

    // println!("cb_mask {:?} {:?}", cb[0], cb_mask(cb[0]));

    Ok(match cb_mask {
        1 => (rem, Offset::Literal { length: 1 + q }),
        3..=8 => {
            let (rem, r) = bytes::complete::take(1u8)(rem)?;
            // println!("RR: [{:08b}:{:02X}] ", r[0], r[0]);
            (
                rem,
                Offset::Dictionary {
                    length: cb_mask,
                    offset: ((q << 8) + r[0] as usize + 1),
                },
            )
        }
        9 => {
            let (rem, r) = bytes::complete::take(1u8)(rem)?;
            let (rem, s) = bytes::complete::take(1u8)(rem)?;
            (
                rem,
                Offset::Dictionary {
                    length: 9 + r[0] as usize,
                    offset: ((q << 8) + s[0] as usize + 1),
                },
            )
        }
        _ => panic!(),
    })
}

#[test]
fn test_get_control_bytes() {
    assert_eq!(
        &format!("{:?}", get_control_bytes(&[0x02]).unwrap()),
        "([], Literal { length: 3 })"
    );

    assert_eq!(
        &format!("{:?}", get_control_bytes(&[0x20, 0x0E]).unwrap()),
        "([], Dictionary { length: 3, offset: 15 })"
    );

    assert_eq!(
        &format!("{:?}", get_control_bytes(&[0x60, 0x00]).unwrap()),
        "([], Dictionary { length: 5, offset: 1 })"
    );
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
