use binread::{io::Cursor, BinReaderExt, NullString, NullWideString};
use std::io::prelude::*;
use std::io::Read;

// #[derive(Debug)]
// pub struct KontaktPreset {}

pub fn kontakt_preset(i: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
