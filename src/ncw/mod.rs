use std::fs::File;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};

use crate::read_bytes::ReadBytesExt;
use crate::Error;

const HEADER_SIZE: usize = 120;
const BLOCK_HEADER_SIZE: usize = 16;

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
    signature: u32,
    base_value: i32,
    bits: i16,
    flags: u16,
}

impl Header {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let mut reader = Cursor::new(reader.read_bytes(HEADER_SIZE)?);

        let magic = reader.read_u64_be()?;
        assert_eq!(magic, 0x01A89ED631010000);

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
        Ok(BlockHeader {
            signature: block_reader.read_u32_le()?,
            base_value: block_reader.read_i32_le()?,
            bits: block_reader.read_i16_le()?,
            flags: block_reader.read_u16_le()?,
        })
    }
}

fn read_block_data(file: &mut File, header: &BlockHeader) -> Result<Vec<i16>, Error> {
    let mut data = Vec::new();

    let block_size = 512 * header.bits as usize / 8;
    let mut block_data = vec![0; block_size];

    file.read_exact(&mut block_data)?;

    if header.bits > 0 {
        // Delta decode
        let mut sample = header.base_value;
        for delta in &block_data {
            sample += *delta as i32;
            data.push(sample as i16);
        }
    } else if header.bits < 0 {
        // Bit truncation
        let bits = (-header.bits) as u8;
        for sample in block_data {
            let truncated = sample & (2u8.pow(bits as u32) - 1);
            data.push(truncated as i16);
        }
    } else {
        // No compression
        data.extend_from_slice(&block_data.iter().map(|b| *b as i16).collect::<Vec<_>>());
    }

    if header.flags == 1 {
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

    let mut block_offsets = Vec::new();
    for _ in 1..num_blocks {
        block_offsets.push(file.read_u32_le()?);
    }
    dbg!(&block_offsets);

    let mut out_file = File::create("decompressed.pcm")?;
    for (i, offset) in block_offsets.iter().enumerate() {
        println!("decompressing block [{i}/{num_blocks}]");

        // Seek to blocks offset
        let block_offset = header.block_headers_offset + offset;
        file.seek(SeekFrom::Start(block_offset as u64))?;
        let header = BlockHeader::read(&mut file)?;
        let data = read_block_data(&mut file, &header)?;

        // Write decompressed PCM data to new file
        for sample in data {
            out_file.write_all(&sample.to_le_bytes())?;
        }
    }

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
