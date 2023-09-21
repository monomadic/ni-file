use std::io::{self, Read, Seek};

#[derive(thiserror::Error, Debug)]
pub enum ReadBytesError {
    #[error("Generic error: {0}")]
    Generic(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),
}

pub trait FromBytes: Sized {
    fn from_be_bytes(bytes: &[u8]) -> Self;
    fn from_le_bytes(bytes: &[u8]) -> Self;
}

macro_rules! impl_from_bytes {
    ($($t:ty),*) => {
        $(
            impl FromBytes for $t {
                fn from_be_bytes(bytes: &[u8]) -> Self {
                    let mut array = [0u8; std::mem::size_of::<Self>()];
                    array.copy_from_slice(bytes);
                    <$t>::from_le_bytes(array)
                }

                fn from_le_bytes(bytes: &[u8]) -> Self {
                    let mut array = [0u8; std::mem::size_of::<Self>()];
                    array.copy_from_slice(bytes);
                    <$t>::from_le_bytes(array)
                }
            }
        )*
    };
}

impl_from_bytes!(u8, i8, u16, i16, u32, i32, u64, i64, f32, f64);

pub enum Endian {
    LE,
    BE,
}

/// Extensions to io::Read for simplifying reading bytes.
pub trait ReadBytesExt: Read + Seek {
    /// Read a number of bytes (failable)
    fn read_bytes(&mut self, bytes: usize) -> Result<Vec<u8>, ReadBytesError> {
        let mut buf = vec![0u8; bytes];
        self.read_exact(&mut buf)
            .map_err(|_| ReadBytesError::Generic(format!("Failed to read {bytes} bytes")))?;
        Ok(buf)
    }

    /// Read a generic big-endian type
    fn read_be_bytes<T: FromBytes>(&mut self) -> io::Result<T> {
        let mut buf = vec![0u8; std::mem::size_of::<T>()];
        self.read_exact(&mut buf)?;
        Ok(T::from_be_bytes(&buf))
    }

    /// Read a generic little-endian type
    fn read_le_bytes<T: FromBytes>(&mut self) -> io::Result<T> {
        let mut buf = vec![0u8; std::mem::size_of::<T>()];
        self.read_exact(&mut buf)?;
        Ok(T::from_le_bytes(&buf))
    }

    fn read_bool(&mut self) -> io::Result<bool> {
        Ok(self.read_le_bytes::<u8>()? == 1)
    }

    fn read_u16_le(&mut self) -> io::Result<u16> {
        self.read_le_bytes::<u16>()
    }

    fn read_u8(&mut self) -> io::Result<u8> {
        self.read_le_bytes::<u8>()
    }

    fn read_i8(&mut self) -> io::Result<i8> {
        self.read_le_bytes::<i8>()
    }

    fn read_u16_be(&mut self) -> io::Result<u16> {
        self.read_be_bytes::<u16>()
    }

    fn read_i16_le(&mut self) -> io::Result<i16> {
        self.read_le_bytes::<i16>()
    }

    fn read_u32_le(&mut self) -> io::Result<u32> {
        self.read_le_bytes::<u32>()
    }

    fn read_i32_be(&mut self) -> io::Result<i32> {
        self.read_le_bytes::<i32>()
    }

    fn read_u32_be(&mut self) -> io::Result<u32> {
        self.read_le_bytes::<u32>()
    }

    fn read_i32_le(&mut self) -> io::Result<i32> {
        self.read_le_bytes::<i32>()
    }

    fn read_f32_le(&mut self) -> io::Result<f32> {
        self.read_le_bytes::<f32>()
    }

    fn read_f64_le(&mut self) -> io::Result<f64> {
        self.read_le_bytes::<f64>()
    }

    fn read_u64_le(&mut self) -> io::Result<u64> {
        self.read_le_bytes::<u64>()
    }

    fn read_u64_be(&mut self) -> io::Result<u64> {
        self.read_be_bytes::<u64>()
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
        String::from_utf8(bytes)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.utf8_error()))
    }

    fn read_widestring_utf16(&mut self) -> Result<String, ReadBytesError> {
        let size_field = self.read_u32_le()?;
        if size_field == 0 {
            return Ok(String::new());
        }

        let buf = self.read_bytes(size_field as usize * 2)?;

        let bytes: Vec<u16> = buf
            .chunks_exact(2)
            .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
            .collect();

        // String::from_utf16(bytes.as_slice())
        //     .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))

        String::from_utf16(bytes.as_slice()).map_err(|e| {
            ReadBytesError::Generic(format!("Error converting bytes to UTF16: {e}, {bytes:?}"))
        })
    }
}
impl<R: Read + Seek + ?Sized> ReadBytesExt for R {}

#[cfg(test)]
mod tests {
    use super::ReadBytesExt;
    use std::io;

    #[test]
    fn test_read_u32_le() {
        let bytes: &[u8] = &[32_u8, 1, 4, 56, 6, 6, 90, 4, 7];
        let mut cursor = io::Cursor::new(bytes);

        let num = cursor.read_u32_le().unwrap();
        assert_eq!(num, 939786528);

        let num = cursor.read_u32_le().unwrap();
        assert_eq!(num, 73008646);
    }

    // #[test]
    // fn test_read_sized_data() {
    //     let bytes: &[u8] = &[9, 0, 0, 0, 0, 0, 0, 0, 4, 5];
    //     let mut cursor = io::Cursor::new(bytes);
    //     let content = cursor.read_sized_data().unwrap();
    //
    //     assert_eq!(content, [9, 0, 0, 0, 0, 0, 0, 0, 4]);
    //     assert_eq!(bytes, [5]);
    //
    //     // test two
    //     let bytes = [
    //         12_u64.to_le_bytes().to_vec(),
    //         64_u32.to_le_bytes().to_vec(),
    //         24_u32.to_le_bytes().to_vec(),
    //     ]
    //     .concat();
    //     assert_eq!(
    //         io::Cursor::new(bytes).read_sized_data().unwrap(),
    //         [12_u64.to_le_bytes().to_vec(), 64_u32.to_le_bytes().to_vec()].concat()
    //     );
    // }
}
