use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

use crate::read_bytes::ReadBytesExt;
use crate::Error;

const HEADER_SIZE: usize = 120;
const BLOCK_HEADER_SIZE: usize = 16;

struct Header {
    channels: u16,
    bits_per_sample: u16,
    sample_rate: u32,
    num_samples: u32,
    blocks_offset: u32,
    data_offset: u32,
    data_size: u32,
}

struct BlockHeader {
    signature: u32,
    base_value: i32,
    bits: i16,
    flags: u16,
}

impl Header {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let mut header_buf = [0; HEADER_SIZE];
        reader.read_exact(&mut header_buf).unwrap();

        Ok(Header {
            channels: reader.read_u16_le()?,
            bits_per_sample: reader.read_u16_le()?,
            sample_rate: reader.read_u32_le()?,
            num_samples: reader.read_u32_le()?,
            blocks_offset: reader.read_u32_le()?,
            data_offset: reader.read_u32_le()?,
            data_size: reader.read_u32_le()?,
        })
    }
}

impl BlockHeader {
    pub fn read<R: ReadBytesExt>(
        mut reader: R,
        num_blocks: usize,
    ) -> Result<Vec<BlockHeader>, Error> {
        let mut headers = Vec::with_capacity(num_blocks);

        for _ in 0..num_blocks {
            let mut header_buf = [0; BLOCK_HEADER_SIZE];
            reader.read_exact(&mut header_buf).unwrap();

            headers.push(BlockHeader {
                signature: reader.read_u32_le()?,
                base_value: reader.read_i32_le()?,
                bits: reader.read_i16_le()?,
                flags: reader.read_u16_le()?,
            });
        }

        Ok(headers)
    }
}

fn read_block_data(file: &mut File, headers: &[BlockHeader]) -> Vec<i16> {
    let mut data = Vec::new();

    for header in headers {
        let block_size = 512 * header.bits as usize / 8;
        let mut block_data = vec![0; block_size];

        file.read_exact(&mut block_data).unwrap();

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
    }

    data
}

pub fn decode(filename: &str) -> Result<(), Error> {
    let mut file = File::open(filename).unwrap();

    // Read header
    let header = Header::read(&mut file)?;

    // Seek to blocks offset
    file.seek(SeekFrom::Start(header.blocks_offset as u64))
        .unwrap();

    // Read block headers
    let num_blocks = (header.data_offset - header.blocks_offset) / 4;
    let headers = BlockHeader::read(&mut file, num_blocks as usize)?;

    // Seek to data offset
    file.seek(SeekFrom::Start(header.data_offset as u64))
        .unwrap();

    // Read block data
    let data = read_block_data(&mut file, &headers);

    // Write decompressed PCM data to new file
    let mut out_file = File::create("decompressed.pcm")?;
    for sample in data {
        out_file.write_all(&sample.to_le_bytes()).unwrap();
    }

    Ok(())
}
