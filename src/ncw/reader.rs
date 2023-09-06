use std::io::{Cursor, Read, Seek, SeekFrom};

use crate::{read_bytes::ReadBytesExt, Error};

const HEADER_SIZE: usize = 120;
const BLOCK_HEADER_SIZE: usize = 16;

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
        dbg!(&header);

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

    pub fn read_block(&mut self) -> Result<Vec<i16>, Error> {
        let mut samples = Vec::new();

        for block_offset in &self.block_offsets {
            self.reader.seek(SeekFrom::Start(
                (self.header.data_offset + block_offset) as u64,
            ))?;

            let block_header = BlockHeader::read(&mut self.reader)?;

            if block_header.bits > 0 {
                // Delta decode, block_data represents the delta from base_value
                let bits = block_header.bits as usize;
                let block_data = self.reader.read_bytes(bits * 64)?;

                samples.append(&mut decode_delta_block_i16(
                    block_header.base_value,
                    &block_data,
                    bits,
                ));

                // let mut read_buffer = [0; 8]; // Max 64 bits per read
                // self.reader
                //     .read_exact(&mut read_buffer[..((bits + 7) / 8)])?;
                //
                // let mut read_idx = 0;
                // let mut current_base = block_header.base_value;
                // while read_idx < read_buffer.len() * 8 {
                //     let sample = read_buffer[read_idx / 8] as i16;
                //     let delta = sample & ((1 >> bits) - 1);
                //     current_base += delta as i32;
                //     samples.push(current_base as i16);
                //     read_idx += bits;
                // }
            } else if block_header.bits < 0 {
                // Bit truncation (simple compression)

                let bits = block_header.bits.abs() as usize;
                let block_data = self.reader.read_bytes(bits * 64)?;

                samples.append(&mut decode_truncated_block_i16(&block_data, bits));

                // for byte in block_data {
                //     let truncated = byte & (2u8.pow(bits) - 1);
                //     samples.push(truncated as i16);
                // }

                // let mut read_buffer = [0; 8]; // Max 64 bits per read
                // self.reader
                //     .read_exact(&mut read_buffer[..((num_bits + 7) / 8)])?;

                // let mut read_idx = 0;
                // while read_idx < read_buffer.len() * 8 {
                //     let sample = (read_buffer[read_idx / 8] as i16) & ((1 << num_bits) - 1);
                //     samples.push(sample);
                //     read_idx += num_bits;
                // }
            } else {
                // No compression
                let bytes_per_sample = self.header.bits_per_sample as usize / 8;

                for _ in 0..512 {
                    let sample_bytes = self.reader.read_bytes(bytes_per_sample).unwrap();
                    let sample = i16::from_le_bytes(sample_bytes.try_into().unwrap());
                    samples.push(sample);
                }
            }

            // self.current_block += 1;
        }
        Ok(samples)
    }

    // pub fn read_samples(&mut self) -> Result<Vec<i16>, Error> {
    //     let mut output = Vec::new();
    //     while let Some(samples) = self.read_block()? {
    //         for sample in samples {
    //             output.push(sample);
    //         }
    //     }
    //     Ok(output)
    // }
}

fn decode_delta_block_i16(base_sample: i32, deltas: &[u8], bit_size: usize) -> Vec<i16> {
    let mut samples: Vec<i16> = Vec::new();
    let mut bit_offset = 0;
    let mut current_base = base_sample.abs() as u32;

    samples.push(current_base as i16);

    while bit_offset + bit_size <= (deltas.len() * 8) {
        let byte_offset = bit_offset / 8;
        let bit_remainder = bit_offset % 8;

        let mut temp: usize = 0;
        for i in 0..((bit_size + 7) / 8) {
            temp |= (deltas[(byte_offset + i) as usize] as usize) << (i * 8);
        }

        let delta = (temp >> bit_remainder) & ((1 << bit_size) - 1);
        current_base += delta as u32;
        samples.push(current_base as i16);

        bit_offset += bit_size;
    }

    samples
}

fn decode_truncated_block_i16(data: &[u8], bit_size: usize) -> Vec<i16> {
    let mut samples: Vec<i16> = Vec::new();
    let mut bit_offset = 0;

    while bit_offset + bit_size <= (data.len() * 8) {
        let byte_offset = bit_offset / 8;
        let bit_remainder = bit_offset % 8;

        let mut temp: u32 = 0;
        for i in 0..((bit_size + 7) / 8) {
            temp |= (data[byte_offset as usize + i as usize] as u32) << (i * 8);
        }
        let value = (temp >> bit_remainder) & ((1 << bit_size) - 1);
        samples.push(value as i16);

        bit_offset += bit_size;
    }

    samples
}

fn read_packed_values_u32(data: &[u8], bit_size: u32) -> Vec<u32> {
    let mut values: Vec<u32> = Vec::new();
    let mut bit_offset = 0;

    while bit_offset + bit_size <= (data.len() * 8) as u32 {
        let byte_offset = bit_offset / 8;
        let bit_remainder = bit_offset % 8;

        let mut temp: u32 = 0;
        for i in 0..((bit_size + 7) / 8) {
            temp |= (data[byte_offset as usize + i as usize] as u32) << (i * 8);
        }
        let value = (temp >> bit_remainder) & ((1 << bit_size) - 1);
        values.push(value);

        bit_offset += bit_size;
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
        for sample in ncw.read_block()? {
            let bytes = sample.to_le_bytes();
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
        for sample in ncw.read_block()? {
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
