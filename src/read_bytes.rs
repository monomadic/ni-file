use std::io;

/// Extensions to io::Read for simplifying reading bytes.
pub trait ReadBytesExt: io::Read {
    fn read_u8(&mut self) -> io::Result<u8> {
        let mut buf = [0u8; 1];
        self.read_exact(&mut buf)?;
        Ok(u8::from_le_bytes(buf))
    }

    fn read_u16_le(&mut self) -> io::Result<u16> {
        let mut buf = [0u8; 2];
        self.read_exact(&mut buf)?;
        Ok(u16::from_le_bytes(buf))
    }

    fn read_u32_le(&mut self) -> io::Result<u32> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }

    fn read_i32_le(&mut self) -> io::Result<i32> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        Ok(i32::from_le_bytes(buf))
    }

    fn read_f32_le(&mut self) -> io::Result<f32> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        Ok(f32::from_le_bytes(buf))
    }

    fn read_f64_le(&mut self) -> io::Result<f64> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf)?;
        Ok(f64::from_le_bytes(buf))
    }

    fn read_u64_le(&mut self) -> io::Result<u64> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf)?;
        Ok(u64::from_le_bytes(buf))
    }

    /// read a number of bytes (failable)
    fn read_bytes(&mut self, bytes: usize) -> io::Result<Vec<u8>> {
        log::info!("reading {} bytes", bytes);
        let mut buf = vec![0u8; bytes];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }

    /// checks data is a valid size and returns its content as a byte array
    fn read_sized_data(&mut self) -> io::Result<Vec<u8>> {
        log::debug!("Reading Sized Data");

        let size_field = self.read_u64_le()?;
        log::debug!("Size Field: {}", size_field);

        // read data into buffer
        let size_field_len = std::mem::size_of::<u64>();
        let buf = self.read_bytes(size_field as usize - size_field_len)?;
        let buf = buf.as_slice();

        Ok([&size_field.to_le_bytes(), buf].concat())
    }

    fn read_string_utf8(&mut self) -> io::Result<String> {
        let mut bytes = Vec::new();
        loop {
            let mut byte = [0];
            self.read_exact(&mut byte)?;
           match byte {
               [0] => break,
               _ => bytes.push(byte[0]),
           }
        }

        // TODO: remove unwrap
        Ok(String::from_utf8(bytes).unwrap())
    }

    fn read_widestring_utf16(&mut self) -> io::Result<String> {
        let size_field = self.read_u32_le()?;
        if size_field == 0 {
            return Ok(String::new());
        }

        let buf = self.read_bytes(size_field as usize * 2)?;

        let u16buf: Vec<u16> = buf
            .chunks_exact(2)
            .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
            .collect();

        Ok(String::from_utf16(u16buf.as_slice()).unwrap())
    }
}
impl<R: io::Read + ?Sized> ReadBytesExt for R {}

#[cfg(test)]
mod tests {
    use super::ReadBytesExt;
    use crate::utils::setup_logger;

    #[test]
    fn test_read_u32_le() {
        let mut bytes: &[u8] = &[32_u8, 1, 4, 56, 6, 6, 90, 4, 7];
        let num = bytes.read_u32_le().unwrap();

        assert_eq!(num, 939786528);
        assert_eq!(bytes, [6, 6, 90, 4, 7]);

        let num = bytes.read_u32_le().unwrap();
        assert_eq!(num, 73008646);
        assert_eq!(bytes, [7]);
    }

    #[test]
    fn test_scan_32() -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    #[test]
    fn test_read_sized_data() {
        setup_logger();

        let mut bytes: &[u8] = &[9, 0, 0, 0, 0, 0, 0, 0, 4, 5];
        let content = bytes.read_sized_data().unwrap();

        assert_eq!(content, [9, 0, 0, 0, 0, 0, 0, 0, 4]);
        assert_eq!(bytes, [5]);

        // test two
        let bytes = [
            12_u64.to_le_bytes().to_vec(),
            64_u32.to_le_bytes().to_vec(),
            24_u32.to_le_bytes().to_vec(),
        ]
        .concat();
        assert_eq!(
            bytes.as_slice().read_sized_data().unwrap(),
            [12_u64.to_le_bytes().to_vec(), 64_u32.to_le_bytes().to_vec()].concat()
        );
    }
}
