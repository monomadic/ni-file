use std::io;

pub trait ReadBytesExt: io::Read {
    fn read_u32_le(&mut self) -> io::Result<u32> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }
}
impl<R: io::Read + ?Sized> ReadBytesExt for R {}

#[cfg(test)]
mod tests {
    use super::ReadBytesExt;

    #[test]
    fn test_read_u32_le() {
        let mut bytes: &[u8] = &[32_u8, 1, 4, 56, 6];
        let num = bytes.read_u32_le().unwrap();

        assert_eq!(num, 939786528);
        assert_eq!(bytes, [6]);
    }
}
