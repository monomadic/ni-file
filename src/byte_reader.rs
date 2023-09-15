use std::{
    any::TypeId,
    io::{Cursor, Read, Result},
};

enum LE {}
enum BE {}

trait Endian {}
impl Endian for LE {}
impl Endian for BE {}

trait ReadBytes: Read {
    fn read_bytes(&mut self, size: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0u8; size];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }

    fn read_as<T: ToEndian, E: Endian>(&mut self) -> Result<T>
    where
        E: 'static,
        T: Sized + Default + Copy,
    {
        let size = std::mem::size_of::<T>();
        let mut buf = vec![0u8; size];
        self.read_exact(&mut buf)?;

        let cursor = Cursor::new(buf);
        let mut value = T::default();
        unsafe {
            std::ptr::copy_nonoverlapping(
                cursor.get_ref().as_ptr() as *const _,
                &mut value as *mut _ as *mut u8,
                size,
            );
        }

        if TypeId::of::<E>() == TypeId::of::<LE>() {
            value = value.to_le();
        } else if TypeId::of::<E>() == TypeId::of::<BE>() {
            value = value.to_be();
        }

        Ok(value)
    }
}

impl<R: Read> ReadBytes for R {}

trait ToEndian {
    fn to_le(self) -> Self;
    fn to_be(self) -> Self;
}

macro_rules! impl_to_endian {
    ($($t:ty),*) => {
        $(
            impl ToEndian for $t {
                fn to_le(self) -> Self {
                    <$t>::to_le(self)
                }

                fn to_be(self) -> Self {
                    <$t>::to_be(self)
                }
            }
        )*
    };
}

impl_to_endian!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_u32_little_endian() -> Result<()> {
        let mut reader = Cursor::new(vec![0x04, 0x03, 0x02, 0x01]);
        let value: u32 = reader.read_as::<u32, LE>()?;
        assert_eq!(value, 0x01020304);
        Ok(())
    }

    #[test]
    fn test_read_u16_little_endian() -> Result<()> {
        let mut reader = Cursor::new(vec![0x02, 0x01]);
        let value: u16 = reader.read_as::<u16, LE>()?;
        assert_eq!(value, 0x0102);
        Ok(())
    }

    #[test]
    fn test_read_u16_big_endian() -> Result<()> {
        let mut reader = Cursor::new(vec![0x02, 0x01]);
        let value: u16 = reader.read_as::<u16, BE>()?;
        assert_eq!(value, 0x0201);
        Ok(())
    }

    // Add more test cases for BigEndian and other types
}
