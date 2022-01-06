use byteorder::{LittleEndian, ReadBytesExt};
use crate::Error;

// #[derive(Debug)]
// pub struct KontaktPreset {}

pub fn read(mut buf: &[u8]) -> Result<(), Error> {
    info!("reading kontakt preset {} bytes", buf.len());
    let a = buf.read_u32::<LittleEndian>()?;
    let b = buf.read_u32::<LittleEndian>()?;
    let c = buf.read_u32::<LittleEndian>()?;
    let size = buf.read_u32::<LittleEndian>()?;
    info!("size {:?}", size);

    let d= buf.read_u32::<LittleEndian>()?;
    let e= buf.read_u16::<LittleEndian>()?;
    info!("unknowns {:?}", (a, b,c, d, e));
    let size2= buf.read_u32::<LittleEndian>()?;
    info!("size2 {:?}", size2);

    // let (mut buf, eof) = buf.split_at(size as usize);

    let (_unknown, mut buf) = buf.split_at(96);


    let strings = buf.read_u32::<LittleEndian>()?;
    info!("found {} strings", strings);

    let s = buf.read_u32::<LittleEndian>()?;
    info!("found {}", s);

    for _ in 0..strings {
        let (string, r) = crate::strings::pascal_string_utf16(buf)?;
        let buf = r;
        info!("found {}", string);
    }



    Ok(())
}
