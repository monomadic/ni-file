//
// NCW is actually just Differential PCM (DPCM).
//

mod bitreader;
mod new;
mod reader;

use hound::{WavSpec, WavWriter};
use std::io::{Read, Seek, Write};

use crate::Error;

use self::reader::NcwReader;

pub fn write_wav<R: Read + Seek, W: Write + Seek>(
    reader: &mut R,
    writer: &mut W,
) -> Result<(), Error> {
    let mut reader = NcwReader::read(reader)?;

    let spec = WavSpec {
        channels: reader.header.channels,
        sample_rate: reader.header.sample_rate,
        bits_per_sample: reader.header.bits_per_sample,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = WavWriter::new(writer, spec)?;

    for sample in reader.read_block()? {
        writer.write_sample(sample as i16)?;
    }
    writer.finalize()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::Cursor};

    #[test]
    fn test_read_16bit_mono() -> Result<(), Error> {
        let mut file = File::open("test-data/NCW/16-bit.ncw")?;
        let mut buffer = Cursor::new(Vec::new());
        write_wav(&mut file, &mut buffer)?;
        Ok(())
    }

    #[test]
    fn test_read_24bit_stereo() -> Result<(), Error> {
        let mut file = File::open("test-data/NCW/24-bit.ncw")?;
        let mut buffer = Cursor::new(Vec::new());
        write_wav(&mut file, &mut buffer)?;
        Ok(())
    }
}
