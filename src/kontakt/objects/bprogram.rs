use crate::{kontakt::structured_object::StructuredObject, read_bytes::ReadBytesExt, Error};

pub struct BProgram {}

impl From<StructuredObject> for BProgram {
    fn from(so: StructuredObject) -> Self {
        todo!()
    }
}

pub struct ProgramPrivateParams {}

impl ProgramPrivateParams {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        // assume v80 for now

        let version = reader.read_u32_le()?;
        println!("version {:?}", version);
        assert!(
            version < 2,
            "ProgramPrivateParams unsupported version: {version}"
        );

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

        // BFileName - SER::ReadBFNTrns
        let filename = reader.read_i32_le()?;
        if filename > 0 {
            todo!();
        }

        // BHeapArray<>
        println!("array items: {:?}", reader.read_u32_le()?);

        Ok(Self {})
    }
}

#[cfg(test)]
mod tests {
    use crate::Error;

    use super::*;

    #[test]
    fn test_private_params_v80() -> Result<(), Error> {
        let mut file = include_bytes!("../tests/Program/v80/private_params/000").as_slice();
        let params = ProgramPrivateParams::read(&mut file)?;
        Ok(())
    }
}
