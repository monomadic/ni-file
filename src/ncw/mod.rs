//
// NCW is actually just Differential PCM (DPCM), and is not proprietary technology.
//
mod bitreader;
mod new;
mod reader;

use hound::{WavSpec, WavWriter};
use std::{
    fs::File,
    io::{Read, Seek},
};

use crate::Error;

use self::reader::NcwReader;

pub fn write_wav<R: Read + Seek>(reader: &mut R, outfile: &mut File) -> Result<(), Error> {
    let mut reader = NcwReader::read(reader)?;

    let spec = WavSpec {
        channels: reader.header.channels,
        sample_rate: reader.header.sample_rate,
        bits_per_sample: reader.header.bits_per_sample,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = WavWriter::new(outfile, spec).unwrap();

    for sample in reader.read_block()? {
        writer.write_sample(sample as i16).unwrap();
    }

    writer.finalize().unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_read_16bit() -> Result<(), Error> {
        let mut file = File::open("test-data/NCW/16-bit.ncw")?;
        write_wav(&mut file, &mut File::create("16.wav")?)?;
        Ok(())
    }

    #[test]
    fn test_read_24bit() -> Result<(), Error> {
        let mut file = File::open("test-data/NCW/24-bit.ncw")?;
        write_wav(&mut file, &mut File::create("16.wav")?)?;
        Ok(())
    }
}
