use crate::{read_bytes::ReadBytesExt, Error};

#[derive(Debug)]
pub struct FM8Matrix;

impl FM8Matrix {
    pub fn print<R: ReadBytesExt>(mut reader: R) -> Result<(), Error> {
        println!("FM Matrix A-OUT1 {:?}", reader.read_i32_le()?);
        println!("FM Matrix A-OUT2 {:?}", reader.read_i32_le()?);
        println!("FM Matrix A-IN {:?}", reader.read_i32_le()?);
        println!("FM Matrix B-A {:?}", reader.read_i32_le()?);
        println!("FM Matrix B-B {:?}", reader.read_i32_le()?);
        println!("FM Matrix B-C {:?}", reader.read_i32_le()?);
        println!("FM Matrix B-D {:?}", reader.read_i32_le()?);
        println!("FM Matrix B-E {:?}", reader.read_i32_le()?);
        println!("FM Matrix B-F {:?}", reader.read_i32_le()?);
        println!("FM Matrix B-X {:?}", reader.read_i32_le()?);
        println!("FM Matrix B-Z {:?}", reader.read_i32_le()?);

        println!("FM Matrix B-OUT1 {:?}", reader.read_i32_le()?);
        println!("FM Matrix B-OUT2 {:?}", reader.read_i32_le()?);
        println!("FM Matrix B-IN {:?}", reader.read_i32_le()?);
        println!("FM Matrix C-A {:?}", reader.read_i32_le()?);
        println!("FM Matrix C-B {:?}", reader.read_i32_le()?);
        println!("FM Matrix C-C {:?}", reader.read_i32_le()?);
        println!("FM Matrix C-D {:?}", reader.read_i32_le()?);
        println!("FM Matrix C-E {:?}", reader.read_i32_le()?);
        println!("FM Matrix C-F {:?}", reader.read_i32_le()?);
        println!("FM Matrix C-X {:?}", reader.read_i32_le()?);
        println!("FM Matrix C-Z {:?}", reader.read_i32_le()?);

        println!("FM Matrix C-OUT1 {:?}", reader.read_i32_le()?);
        println!("FM Matrix C-OUT2 {:?}", reader.read_i32_le()?);
        println!("FM Matrix C-IN {:?}", reader.read_i32_le()?);
        println!("FM Matrix D-A {:?}", reader.read_i32_le()?);
        println!("FM Matrix D-B {:?}", reader.read_i32_le()?);
        println!("FM Matrix D-C {:?}", reader.read_i32_le()?);
        println!("FM Matrix D-D {:?}", reader.read_i32_le()?);
        println!("FM Matrix D-E {:?}", reader.read_i32_le()?);
        println!("FM Matrix D-F {:?}", reader.read_i32_le()?);
        println!("FM Matrix D-X {:?}", reader.read_i32_le()?);
        println!("FM Matrix D-Z {:?}", reader.read_i32_le()?);

        println!("FM Matrix D-OUT1 {:?}", reader.read_i32_le()?);
        println!("FM Matrix D-OUT2 {:?}", reader.read_i32_le()?);
        println!("FM Matrix D-IN {:?}", reader.read_i32_le()?);
        println!("FM Matrix E-A {:?}", reader.read_i32_le()?);
        println!("FM Matrix E-B {:?}", reader.read_i32_le()?);
        println!("FM Matrix E-C {:?}", reader.read_i32_le()?);
        println!("FM Matrix E-D {:?}", reader.read_i32_le()?);
        println!("FM Matrix E-E {:?}", reader.read_i32_le()?);
        println!("FM Matrix E-F {:?}", reader.read_i32_le()?);
        println!("FM Matrix E-X {:?}", reader.read_i32_le()?);
        println!("FM Matrix E-Z {:?}", reader.read_i32_le()?);

        println!("FM Matrix E-OUT1 {:?}", reader.read_i32_le()?);
        println!("FM Matrix E-OUT2 {:?}", reader.read_i32_le()?);
        println!("FM Matrix E-IN {:?}", reader.read_i32_le()?);
        println!("FM Matrix F-A {:?}", reader.read_i32_le()?);
        println!("FM Matrix F-B {:?}", reader.read_i32_le()?);
        println!("FM Matrix F-C {:?}", reader.read_i32_le()?);
        println!("FM Matrix F-D {:?}", reader.read_i32_le()?);
        println!("FM Matrix F-E {:?}", reader.read_i32_le()?);
        println!("FM Matrix F-F {:?}", reader.read_i32_le()?);
        println!("FM Matrix F-X {:?}", reader.read_i32_le()?);
        println!("FM Matrix F-Z {:?}", reader.read_i32_le()?);

        println!("FM Matrix F-OUT1 {:?}", reader.read_i32_le()?);
        println!("FM Matrix F-OUT2 {:?}", reader.read_i32_le()?);
        println!("FM Matrix F-IN {:?}", reader.read_i32_le()?);
        println!("FM Matrix X-A {:?}", reader.read_i32_le()?);
        println!("FM Matrix X-B {:?}", reader.read_i32_le()?);
        println!("FM Matrix X-C {:?}", reader.read_i32_le()?);
        println!("FM Matrix X-D {:?}", reader.read_i32_le()?);
        println!("FM Matrix X-E {:?}", reader.read_i32_le()?);
        println!("FM Matrix X-F {:?}", reader.read_i32_le()?);
        println!("FM Matrix X-X {:?}", reader.read_i32_le()?);
        println!("FM Matrix X-Z {:?}", reader.read_i32_le()?);

        println!("FM Matrix X-OUT1 {:?}", reader.read_i32_le()?);
        println!("FM Matrix X-OUT2 {:?}", reader.read_i32_le()?);
        println!("FM Matrix X-IN {:?}", reader.read_i32_le()?);
        println!("FM Matrix Z-A {:?}", reader.read_i32_le()?);
        println!("FM Matrix Z-B {:?}", reader.read_i32_le()?);
        println!("FM Matrix Z-C {:?}", reader.read_i32_le()?);
        println!("FM Matrix Z-D {:?}", reader.read_i32_le()?);
        println!("FM Matrix Z-E {:?}", reader.read_i32_le()?);
        println!("FM Matrix Z-F {:?}", reader.read_i32_le()?);
        println!("FM Matrix Z-X {:?}", reader.read_i32_le()?);
        println!("FM Matrix Z-Z {:?}", reader.read_i32_le()?);

        println!("FM Matrix Z-OUT1 {:?}", reader.read_i32_le()?);
        println!("FM Matrix Z-OUT2 {:?}", reader.read_i32_le()?);
        println!("FM Matrix Z-IN {:?}", reader.read_i32_le()?);
        println!("FM Matrix IN-A {:?}", reader.read_i32_le()?);
        println!("FM Matrix IN-B {:?}", reader.read_i32_le()?);
        println!("FM Matrix IN-C {:?}", reader.read_i32_le()?);
        println!("FM Matrix IN-D {:?}", reader.read_i32_le()?);
        println!("FM Matrix IN-E {:?}", reader.read_i32_le()?);
        println!("FM Matrix IN-F {:?}", reader.read_i32_le()?);
        println!("FM Matrix IN-X {:?}", reader.read_i32_le()?);
        println!("FM Matrix IN-Z {:?}", reader.read_i32_le()?);

        println!("FM Matrix IN-OUT1 {:?}", reader.read_i32_le()?);
        println!("FM Matrix IN-OUT2 {:?}", reader.read_i32_le()?);
        println!("FM Matrix IN-IN {:?}", reader.read_i32_le()?);
        println!("FM Matrix A-A {:?}", reader.read_i32_le()?);
        println!("FM Matrix A-B {:?}", reader.read_i32_le()?);
        println!("FM Matrix A-C {:?}", reader.read_i32_le()?);
        println!("FM Matrix A-D {:?}", reader.read_i32_le()?);
        println!("FM Matrix A-E {:?}", reader.read_i32_le()?);
        println!("FM Matrix A-F {:?}", reader.read_i32_le()?);
        println!("FM Matrix A-X {:?}", reader.read_i32_le()?);
        println!("FM Matrix A-Z {:?}", reader.read_i32_le()?);

        println!("FM Matrix IN-OUT1 {:?}", reader.read_i32_le()?);
        println!("FM Matrix IN-OUT2 {:?}", reader.read_i32_le()?);
        println!("FM Matrix IN-IN {:?}", reader.read_i32_le()?);
        println!("FM Matrix A-A {:?}", reader.read_i32_le()?);
        println!("FM Matrix A-B {:?}", reader.read_i32_le()?);
        println!("FM Matrix A-C {:?}", reader.read_i32_le()?);
        println!("FM Matrix A-D {:?}", reader.read_i32_le()?);
        println!("FM Matrix A-E {:?}", reader.read_i32_le()?);
        println!("FM Matrix A-F {:?}", reader.read_i32_le()?);
        println!("FM Matrix A-X {:?}", reader.read_i32_le()?);
        println!("FM Matrix A-Z {:?}", reader.read_i32_le()?);
        Ok(())
    }
}
