use crate::read_bytes::{ReadBytesError, ReadBytesExt};

pub trait StringReader: ReadBytesExt {
    fn read_nullterminated_utf16(&mut self) -> Result<String, ReadBytesError> {
        let mut u16_chars = Vec::new();
        let mut buffer = [0u8; 2]; // 2 bytes for each UTF-16 character

        loop {
            let bytes_read = self.read(&mut buffer)?;
            if bytes_read == 0 {
                break; // End of file reached
            }

            let u16_char = u16::from_le_bytes([buffer[0], buffer[1]]);

            if u16_char == 0 {
                break; // Null character found
            }

            u16_chars.push(u16_char);
        }

        Ok(String::from_utf16_lossy(&u16_chars))
    }
}

impl<R: ReadBytesExt> StringReader for R {}
