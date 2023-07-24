use crate::{read_bytes::ReadBytesExt, Error};

#[derive(Debug)]
pub struct MidiArpeggiator;

impl MidiArpeggiator {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        // u32
        // u8 (!= null)
        //
        // i8
        // i8
        // i8
        // i8
        // i8
        // i8
        // i8
        // i8
        // i8
        // i8
        // i8
        // i8
        // i8
        //
        // i8
        // i8
        // i8
        // i8
        //
        // f32
        //
        // i8

        Ok(Self {})
    }
}
