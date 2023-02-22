use std::convert::TryInto;
use std::io;

pub trait ReadBytesExt: io::Read {
    fn read_u8(&mut self) -> io::Result<u8> {
        let mut buf = [0u8; 1];
        self.read_exact(&mut buf)?;
        Ok(u8::from_le_bytes(buf))
    }

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

    /// TODO: take any int instead of just usize
    /// read a number of bytes (failable)
    fn read_bytes(&mut self, bytes: usize) -> io::Result<Vec<u8>> {
        let mut buf = vec![0u8; bytes];
        log::info!("reading {} bytes", buf.len());
        self.read_exact(&mut buf)?;
        Ok(buf)
    }

    /// checks data is a valid size and returns its content as a byte array
    fn read_sized_data(&mut self) -> io::Result<Vec<u8>> {
        // TODO: idea - just create a new pointer to the byte array and read that first, then read
        // the original buffer for the actual data.
        let size_field = self.read_u64_le()?;
        log::debug!("size field: {}", size_field);

        // read data into buffer
        let size_field_len = std::mem::size_of::<u64>();
        let buf = self.read_bytes(size_field as usize - size_field_len)?;
        let buf = buf.as_slice();

        Ok([&size_field.to_le_bytes(), buf].concat())
    }
}
impl<R: io::Read + ?Sized> ReadBytesExt for R {}

// TODO: implement read_widestring
//
// fn pascal_string_utf16<R: Read + Seek>(
//     reader: &mut R,
//     _ro: &binread::ReadOptions,
//     _: (),
// ) -> BinResult<String> {
//     let size: u32 = reader.read_le()?;
//
//     info!("string length {}", size);
//
//     if size == 0 {
//         return Ok(String::new());
//     }
//
//     let string: String = reader.read_le::<binread::NullWideString>()?.into_string();
//
//     Ok(string)
// }

#[cfg(test)]
mod tests {
    use super::ReadBytesExt;
    use crate::utils::setup_logger;

    #[test]
    fn test_read_u32_le() {
        let mut bytes: &[u8] = &[32_u8, 1, 4, 56, 6];
        let num = bytes.read_u32_le().unwrap();

        assert_eq!(num, 939786528);
        assert_eq!(bytes, [6]);
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
