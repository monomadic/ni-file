use byteorder::{LittleEndian, ReadBytesExt};
use crate::Error;
use binread::{io::Cursor, prelude::*};
use std::io::BufRead;

// #[derive(Debug)]
// pub struct KontaktPreset {}

pub fn read(buf: &[u8]) -> Result<(), Error> {
    info!("reading kontakt preset {} bytes", buf.len());
    let mut cursor = Cursor::new(buf);

    let a: u32 = cursor.read_le()?;
    let b: u32 = cursor.read_le()?;
    let c: u32 = cursor.read_le()?;
    info!("unknowns {:?}", (a, b, c));

    let size: u32 = cursor.read_le()?;
    info!("size {:?}", size);

    let d: u32 = cursor.read_le()?;
    let e: u16 = cursor.read_le()?;
    info!("unknowns {:?}", (d, e));

    let size2: u32 = cursor.read_le()?;
    info!("size2 {}", size2);

    let _blob = cursor.consume(96);

    let strings: u32 = cursor.read_le()?;
    info!("found {} (strings?)", strings);

    let f_size: u32 = cursor.read_le()?;
    info!("f_size {}", f_size);

    let title = crate::strings::pascal_string_utf16(&mut cursor)?;
    info!("Title: {}", title);

    let _blob = cursor.consume(42);

    let g_size: u64 = cursor.read_le()?;
    info!("g_size {}", g_size);

    let app = crate::strings::pascal_string_utf16(&mut cursor)?;
    info!("App: {}", app);

    let _blob = cursor.consume(16);

    let h_size: u32 = cursor.read_le()?;
    info!("h_size {}", h_size);

    // jump straight to last section
    let _blob = cursor.consume(h_size as usize - 4);
    info!("cursor pos {}", cursor.position() + 168);

    // MAPPINGS SECTION
    let _blob = cursor.consume(30);
    let space_remaining: u32 = cursor.read_le()?;
    info!("space_remaining {}", space_remaining);

    let _blob = cursor.consume(18);

    let app = crate::strings::pascal_string_utf16(&mut cursor)?;
    info!("App: {}", app);

    info!("cursor pos {}", cursor.position() + 168);
    Ok(())
}
