//
// NCW is basically just Differential PCM (DPCM).
// I'm not sure why NI claimed to have their own format for the sake of it,
// except to lock you in.
//

// mod bitreader;
// mod new;
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
    dbg!(spec);

    let mut writer = WavWriter::new(writer, spec)?;

    for sample in reader.decode_samples()? {
        match reader.header.bits_per_sample {
            32 | 24 => {
                writer.write_sample(sample as i32)?;
            }
            16 => {
                writer.write_sample(sample as i16)?;
            }
            8 => {
                writer.write_sample(sample as i8)?;
            }
            _ => panic!("Unknown output sample format"),
        }
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

        // let mut output_file = File::create("16.wav")?;
        // output_file.write_all(&buffer.into_inner())?;

        Ok(())
    }

    #[test]
    fn test_read_24bit_stereo() -> Result<(), Error> {
        let mut file = File::open("test-data/NCW/24-bit.ncw")?;
        let mut buffer = Cursor::new(Vec::new());
        write_wav(&mut file, &mut buffer)?;

        // let mut output_file = File::create("24.wav")?;
        // output_file.write_all(&buffer.into_inner())?;

        Ok(())
    }
}
