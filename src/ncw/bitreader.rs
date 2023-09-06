// Reads bit-truncated values of arbitrary precision from a packed binary.

use std::io;

use crate::read_bytes::ReadBytesExt;

pub struct BitReader<R: ReadBytesExt> {
    reader: R,
    buffer: u8,
    bits_left: u8,
}

impl<R: ReadBytesExt> BitReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: 0,
            bits_left: 0,
        }
    }

    pub fn read_bits_i16(&mut self, n: usize) -> io::Result<i16> {
        if n > 16 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Cannot read more than 16 bits at a time",
            ));
        }

        let mut out = 0i16;
        let mut bits_read = 0;

        while bits_read < n {
            if self.bits_left == 0 {
                self.buffer = self.reader.read_u8()?;
                self.bits_left = 8;
            }

            let to_read = std::cmp::min(n - bits_read, self.bits_left as usize);

            let mut value = self.buffer >> (self.bits_left - to_read as u8);
            value &= (1 << to_read) - 1; // mask to 19 bits

            out |= (value as i16) << (n - bits_read - to_read); // shift right

            self.buffer <<= to_read;
            self.bits_left -= to_read as u8;
            bits_read += to_read;
        }

        Ok(out)
    }
}

/// Read specified number of bits from data buffer
fn read_bits(data: &[u8], index: usize, num_bits: usize) -> u32 {
    let byte_index = index / 8;
    let bit_index = index % 8;

    let mut value = data[byte_index] as u32;
    value <<= bit_index;

    if num_bits > 8 - bit_index {
        value |= (data[byte_index + 1] as u32) >> (8 - bit_index);
    }

    value & ((1 << num_bits) - 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_read_bits() {
        let data = [0b01010101, 0b10101010];

        // Read 2 bits from offset 0
        assert_eq!(read_bits(&data, 0, 2), 0b01);

        // Read 6 bits from offset 1
        assert_eq!(read_bits(&data, 1, 6), 0b101010);

        // Read 8 bits spanning 2 bytes
        assert_eq!(read_bits(&data, 3, 8), 0b01010101);

        // Read 9 bits (spans bytes)
        assert_eq!(read_bits(&data, 4, 9), 0b101010101);

        // Read from end
        assert_eq!(read_bits(&data, 15, 3), 0b010);
    }

    // #[test]
    // fn test_read_bits_19() -> io::Result<()> {
    //     // Create a mock reader with bytes to read from.
    //     let data = vec![0b11111111, 0b11111111, 0b11111111];
    //     let reader = Cursor::new(data);
    //     let mut bit_reader = BitReader::new(reader);
    //
    //     // Read 19 bits
    //     let result = bit_reader.read_bits_i16(19)?;
    //
    //     // The 19 bits read should be the first 19 bits of the data (all 1s)
    //     assert_eq!(result >> (32 - 19), 0b1111111111111111111);
    //
    //     Ok(())
    // }
    //
    // #[test]
    // fn test_read_bits_exceed_32() {
    //     // Create a mock reader with bytes to read from.
    //     let data = vec![0xFF; 5]; // 5 bytes (40 bits)
    //     let reader = Cursor::new(data);
    //     let mut bit_reader = BitReader::new(reader);
    //
    //     // Try reading more than 32 bits, expect an error
    //     let result = bit_reader.read_bits_i16(33);
    //     assert!(result.is_err());
    // }

    // Add more tests as needed
}
