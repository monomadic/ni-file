use crate::{read_bytes::ReadBytesExt, Error};

#[derive(Debug)]
pub struct PrivParsV80;

impl PrivParsV80 {
    // BProgram::doReadPrivPars
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        // 0x80
        println!("version {:?}", reader.read_i32_le()?); // < 2
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_i32_le()?);
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_i32_le()?);
        println!("{:?}", reader.read_i32_le()?);
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_i32_le()?);
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_i32_le()?);
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_i32_le()?);
        println!("{:?}", reader.read_i32_le()?);
        println!("{:?}", reader.read_i32_le()?);
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_bool()?);
        println!("{:?}", reader.read_i16_le()?);
        println!("{:?}", reader.read_bool()?);

        println!("{:?}", reader.read_i32_le()?);
        println!("{:?}", reader.read_i32_le()?);
        Ok(Self {})
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn test_private_parameters() -> Result<(), Error> {
        // Version 0x80
        let file = File::open("../../tests/patchdata/KontaktV42/priv_params/4.2.2.4504/000")?;
        PrivParsV80::read(file)?;

        // let file = include_bytes!("tests/structured_object/5.3.0.6464/000");
        // let mut file = file.as_slice();
        // StructuredObject::read(&mut file)?;
        // StructuredObject::read(&mut file)?;

        Ok(())
    }
}
