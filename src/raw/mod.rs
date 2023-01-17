/// Raw method
/// - attempts to read files incrementally
/// - safer, better error messages
/// - only parses specific data on demand
///
use crate::prelude::*;

pub struct Frame {
    pub kind: u32,
    pub uuid: [u8; 16],
    pub data: Vec<u8>,
    pub children: Vec<Vec<u8>>,
}

pub fn read(i: &[u8]) -> Result<()> {
    Ok(())
}
