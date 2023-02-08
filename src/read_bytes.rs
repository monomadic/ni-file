use std::convert::TryInto;
use std::io;

pub trait ReadBytesExt: io::Read {
    fn read_u32_le(&mut self) -> io::Result<u32> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }

    fn scan_u32_le(bytes: &[u8]) -> Result<u32, std::array::TryFromSliceError> {
        let buffer: [u8; 4] = bytes.try_into()?;
        let result = u32::from_le_bytes(buffer);
        Ok(result)
    }

    fn read_u64_le(&mut self) -> io::Result<u64> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf)?;
        Ok(u64::from_le_bytes(buf))
    }

    fn scan_u64_le(bytes: &[u8]) -> Result<u64, std::array::TryFromSliceError> {
        let buffer: [u8; 8] = bytes.try_into()?;
        let result = u64::from_le_bytes(buffer);
        Ok(result)
    }

    /// read a number of bytes (failable)
    fn read_bytes(&mut self, bytes: usize) -> io::Result<Vec<u8>> {
        let mut buf = vec![0u8; bytes];
        self.read_exact(&mut buf)?;
        Ok(buf)
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
