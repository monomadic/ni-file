use std::fs::File;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};

use crate::read_bytes::ReadBytesExt;
use crate::Error;

const HEADER_SIZE: usize = 120;
const BLOCK_HEADER_SIZE: usize = 16;
const NCW_SAMPLES: usize = 512;
const MAX_CHANNELS: usize = 6;

#[derive(Debug)]
struct Header {
    channels: u16,
    bits_per_sample: u16,
    sample_rate: u32,
    num_samples: u32,
    block_address_offset: u32,
    block_headers_offset: u32,
    blocks_data_len: u32,
}

#[derive(Debug)]
struct BlockHeader {
    base_value: i32,
    bits: i16,
    flags: u16,
}

impl Header {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let mut reader = Cursor::new(reader.read_bytes(HEADER_SIZE)?);

        let magic = reader.read_u64_be()?;
        assert!([0x01A89ED631010000, 0x01A89ED630010000].contains(&magic));

        Ok(Header {
            channels: reader.read_u16_le()?,
            bits_per_sample: reader.read_u16_le()?,
            sample_rate: reader.read_u32_le()?,
            num_samples: reader.read_u32_le()?,
            block_address_offset: reader.read_u32_le()?,
            block_headers_offset: reader.read_u32_le()?,
            blocks_data_len: reader.read_u32_le()?,
        })
    }
}

impl BlockHeader {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<BlockHeader, Error> {
        let mut block_reader = Cursor::new(reader.read_bytes(BLOCK_HEADER_SIZE)?);

        let magic = block_reader.read_u32_be()?;
        assert_eq!(magic, 0x160C9A3E);

        Ok(BlockHeader {
            base_value: block_reader.read_i32_le()?,
            bits: block_reader.read_i16_le()?,
            flags: block_reader.read_u16_le()?,
        })
    }

    // pub fn read_samples<R: ReadBytesExt>(&self, mut reader: R) -> Result<(), Error> {
    //     match self.bits {
    //         8 => self.read_ncw8(),
    //         16 => self.read_ncw16(),
    //         24 => self.read_ncw24(),
    //         32 => self.read_ncw32(),
    //         _ => panic!("Unsupported bits per sample!"),
    //     }
    // }
}

fn read_block_data<R: ReadBytesExt>(
    mut reader: R,
    header: &Header,
    block_header: &BlockHeader,
) -> Result<Vec<i32>, Error> {
    let num_samples = 512;

    let mut data = Vec::new();
    let bits = block_header.bits as usize;

    // Max 64 bits per read
    // let block_data = reader.read_bytes(bits * 64)?;
    // let mut block_data = vec![0; bits * 64]; // Max 64 bits per read
    // reader.read_exact(&mut block_data[..((bits + 7) / 8)])?;
    // dbg!(block_header.bits);

    if block_header.bits > 0 {
        // Delta decode
        // - block_data represents the delta from base_value
        let block_data = reader.read_bytes(bits * 64)?;

        // convert bits to bytes

        // let sample_size = header.bits_per_sample / 8;
        // let block_size = sample_size * num_samples;
        // let mut reader = Cursor::new(reader.read_bytes(block_size as usize)?);
        // let mut current_base = block_header.base_value;

        for delta in &block_data {
            // println!("{}+{}=", &current_base, delta);
            current_base += *delta as i32;
            // dbg!(current_base);
            data.push(current_base as i32);
        }
    } else if block_header.bits < 0 {
        todo!();
        let block_size = 512 * block_header.bits as usize / 8;
        // Bit truncation
        let bits = (-block_header.bits) as u8;
        for sample in block_data {
            let truncated = sample & (2u8.pow(bits as u32) - 1);
            data.push(truncated as i32);
        }
    } else {
        todo!();
        // No compression
        data.extend_from_slice(&block_data.iter().map(|b| *b as i32).collect::<Vec<_>>());
    }

    if block_header.flags == 1 {
        panic!("mid/side");
        // Mid/side to left/right conversion
        let mid = data.iter().step_by(2).cloned().collect::<Vec<_>>();
        let side = data.iter().skip(1).step_by(2).cloned().collect::<Vec<_>>();

        data = mid
            .iter()
            .zip(side.iter())
            .map(|(m, s)| m + s)
            .chain(mid.iter().zip(side.iter()).map(|(m, s)| m - s))
            .collect();
    }

    Ok(data)
}

pub fn decode(filename: &str) -> Result<(), Error> {
    let mut file = File::open(filename)?;

    // Read header
    let header = Header::read(&mut file)?;
    dbg!(&header);

    // Read block headers
    let num_blocks = (header.block_headers_offset - header.block_address_offset) / 4;
    dbg!(num_blocks);
    dbg!(num_blocks * 512);

    let mut block_offsets = Vec::new();
    for _ in 1..num_blocks {
        block_offsets.push(file.read_u32_le()?);
    }

    assert_eq!(num_blocks as usize, block_offsets.len() + 1);
    // dbg!(&block_offsets);

    //let mut out_file = File::create("decompressed.pcm").unwrap();
    let mut out = Vec::new();
    let mut out_file = File::create("decompressed.pcm")?;
    for (i, offset) in block_offsets.iter().enumerate() {
        // Seek to blocks offset
        let block_offset = header.block_headers_offset + offset;
        let pos = file.seek(SeekFrom::Start(block_offset as u64))?;

        // println!("decompressing block [{i}/{num_blocks}] offset: {offset}/{block_offset}");
        // println!("seek to: {pos}");

        // Read BlockHeader
        let block_header = BlockHeader::read(&mut file)?;
        // dbg!(&block_header);

        let data = read_block_data(&mut file, &block_header)?;
        dbg!(data.len());
        for sample in data {
            out_file.write_all(&sample.to_le_bytes()).unwrap();
            out.append(&mut sample.to_le_bytes().to_vec());
        }
    }

    dbg!(out.len());

    // let blocks_per_channel = num_blocks / 2;
    // let (left_channel, right_channel) = block_offsets.split_at(blocks_per_channel as usize);
    //
    // // Create a new WAV file
    // let spec = hound::WavSpec {
    //     channels: 2,
    //     sample_rate: 44100,
    //     bits_per_sample: 16,
    //     sample_format: hound::SampleFormat::Int,
    // };
    //
    // let mut writer = hound::WavWriter::create("output.wav", spec).unwrap();
    //
    // // let mut out_file = File::create("decompressed.pcm")?;
    // for (i, _) in [1..blocks_per_channel].iter().enumerate() {
    //     println!("decompressing block [{i}/{num_blocks}]");
    //
    //     // Seek to blocks offset
    //     let offset = left_channel[i];
    //     let block_offset = header.block_headers_offset + offset;
    //     file.seek(SeekFrom::Start(block_offset as u64))?;
    //     let header = BlockHeader::read(&mut file)?;
    //     let data = read_block_data(&mut file, &header)?;
    //
    //     let sample_count = data.len() / 2;
    //
    //     // Write decompressed PCM data to new file
    //     for i in 0..sample_count {
    //         // out_file.write_all(&sample.to_le_bytes())?;
    //     }
    // }
    //
    // // Write each PCM sample to the WAV file
    // writer.write_sample(left_channel[i]).unwrap();
    // writer.write_sample(right_channel[i]).unwrap();
    //
    // // Finalize the WAV file
    // writer.finalize().unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() -> Result<(), Error> {
        decode("test-data/NCW/000.ncw")?;
        Ok(())
    }
}
