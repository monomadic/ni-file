use std::io::{Cursor, Read, Seek, SeekFrom};

use crate::{read_bytes::ReadBytesExt, Error};

const HEADER_SIZE: usize = 120;
const BLOCK_HEADER_SIZE: usize = 16;
const NCW_SAMPLES: usize = 512;
const MAX_CHANNELS: usize = 6;

#[derive(Debug)]
pub struct NcwReader<R> {
    pub reader: R,
    pub header: NcwHeader,
    pub block_offsets: Vec<u32>,
    pub current_block: usize,
}

#[derive(Debug)]
pub struct NcwHeader {
    pub channels: u16,
    pub bits_per_sample: u16,
    pub sample_rate: u32,
    pub num_samples: u32,
    pub blocks_offset: u32,
    pub data_offset: u32,
    pub data_size: u32,
}

#[derive(Debug)]
pub struct BlockHeader {
    pub base_value: i32,
    pub bits: i16,
    pub flags: u16,
}

impl<R: Read + Seek> NcwReader<R> {
    pub fn read(mut reader: R) -> Result<Self, Error> {
        let header = NcwHeader::read(&mut reader)?;
        // dbg!(&header);

        let block_offsets_len = header.data_offset - header.blocks_offset;
        let num_blocks = block_offsets_len / 4;

        let mut block_offsets = Vec::new();
        for _ in 1..num_blocks {
            block_offsets.push(reader.read_u32_le()?);
        }

        Ok(Self {
            reader,
            header,
            block_offsets,
            current_block: 0,
        })
    }

    pub fn num_blocks(&self) -> u32 {
        let block_offsets_len = self.header.data_offset - self.header.blocks_offset;
        block_offsets_len / 4
    }

    /// Decode all blocks in 32-bit PCM samples.
    pub fn decode_samples(&mut self) -> Result<Vec<i32>, Error> {
        let mut samples = Vec::new();

        for (i, block_offset) in self.block_offsets.iter().enumerate() {
            // dbg!(i);
            self.reader.seek(SeekFrom::Start(
                (self.header.data_offset + block_offset) as u64,
            ))?;

            let block_header = BlockHeader::read(&mut self.reader)?;
            if block_header.flags == 1 {
                panic!("mid/side");
            }

            let bits = block_header.bits.abs() as usize;
            let block_data_len = bits * 64;
            let mut block_data = vec![0; block_data_len];
            self.reader.read_exact(&mut block_data).unwrap();

            if block_header.bits > 0 {
                // Delta decode, block_data represents the delta from base_value
                samples.append(&mut decode_delta_block_i32(
                    block_header.base_value,
                    &block_data,
                    bits,
                ));
            } else if block_header.bits < 0 {
                // Bit truncation (simple compression)
                let bits = block_header.bits.abs() as usize;
                samples.append(&mut decode_truncated_block_i32(&block_data, bits));
            } else {
                // No compression
                let bytes_per_sample = self.header.bits_per_sample as usize / 8;

                for _ in 0..512 {
                    let sample_bytes = self.reader.read_bytes(bytes_per_sample).unwrap();
                    let sample = i32::from_le_bytes(sample_bytes.try_into().unwrap());
                    samples.push(sample);
                }
            }

            // self.current_block += 1;
        }
        Ok(samples)
    }
}

fn decode_delta_block_i32(base_sample: i32, deltas: &[u8], bits: usize) -> Vec<i32> {
    assert_eq!(deltas.len(), bits * 64);

    let mut samples: Vec<i32> = vec![0; NCW_SAMPLES];
    let mut prev_base = base_sample;
    let delta_values = read_packed_values_i32(deltas, bits);

    for (i, delta) in delta_values.iter().enumerate() {
        samples[i] = prev_base;
        prev_base = prev_base + delta;
    }

    samples
}

fn decode_delta_block_i16(base_sample: i32, deltas: &[u8], bits: usize) -> Vec<i16> {
    let mut samples: Vec<i16> = vec![0; NCW_SAMPLES];
    let mut prev_base = base_sample;

    let delta_values = read_packed_values_i32(deltas, bits);

    for (i, delta) in delta_values.iter().enumerate() {
        samples[i] = prev_base as i16;
        let value = prev_base + delta;
        prev_base = value;
    }

    samples
}

fn decode_truncated_block_i32(data: &[u8], bit_size: usize) -> Vec<i32> {
    let mut samples: Vec<i32> = Vec::new();
    let mut bit_offset = 0;

    while bit_offset + bit_size <= (data.len() * 8) {
        let byte_offset = bit_offset / 8;
        let bit_remainder = bit_offset % 8;

        let mut temp: i32 = 0;
        for i in 0..((bit_size + 7) / 8) {
            temp |= (data[byte_offset as usize + i as usize] as i32) << (i * 8);
        }
        let value = (temp >> bit_remainder) & ((1 << bit_size) - 1);
        samples.push(value);

        bit_offset += bit_size;
    }

    samples
}

fn read_packed_values_i32(data: &[u8], precision_in_bits: usize) -> Vec<i32> {
    let mut values: Vec<i32> = Vec::new();
    let mut bit_accumulator: i32 = 0;
    let mut bits_in_accumulator: usize = 0;
    let mut byte_index = 0;

    while byte_index < data.len() {
        // Accumulate more bits
        bit_accumulator |= (data[byte_index] as i32) << bits_in_accumulator;
        bits_in_accumulator += 8;
        byte_index += 1;

        // Extract values as long as enough bits are available
        while bits_in_accumulator >= precision_in_bits {
            let mut value = bit_accumulator & ((1 << precision_in_bits) - 1);
            if value & (1 << (precision_in_bits - 1)) != 0 {
                value |= !0 << precision_in_bits;
            }
            values.push(value);

            // Remove used bits
            bit_accumulator >>= precision_in_bits;
            bits_in_accumulator -= precision_in_bits;
        }
    }

    values
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
}

impl NcwHeader {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let mut reader = Cursor::new(reader.read_bytes(HEADER_SIZE)?);

        let magic = reader.read_u64_be()?;
        assert!([0x01A89ED631010000, 0x01A89ED630010000].contains(&magic));

        Ok(Self {
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

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;

    use super::*;

    #[test]
    fn test_read_16bit() -> Result<(), Error> {
        let file = File::open("test-data/NCW/16-bit.ncw")?;
        let mut ncw = NcwReader::read(file)?;

        let mut output = File::create("16.pcm")?;
        for sample in ncw.decode_samples()? {
            // let bytes = sample.to_le_bytes();
            let bytes = (sample as i16).to_le_bytes();
            output.write_all(&bytes)?;
        }

        //assert_eq!(ncw.header.num_samples as usize, ncw.read_samples()?.len());

        Ok(())
    }

    #[test]
    fn test_read_24bit() -> Result<(), Error> {
        let file = File::open("test-data/NCW/24-bit.ncw")?;
        let mut ncw = NcwReader::read(file)?;

        let mut bytes: Vec<u8> = Vec::new();
        for sample in ncw.decode_samples()? {
            let low_byte = (sample & 0xFF) as u8;
            let high_byte = ((sample >> 8) & 0xFF) as u8;

            bytes.push(low_byte);
            bytes.push(high_byte);
        }

        let mut output_file = File::create("24.pcm")?;
        output_file.write_all(&bytes)?;

        // assert_eq!(ncw.header.num_samples as usize, ncw.read_samples()?.len());

        Ok(())
    }
}
