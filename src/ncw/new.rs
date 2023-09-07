use std::io::{Cursor, Read, Seek, SeekFrom};

use crate::read_bytes::ReadBytesExt;
use crate::Error;

use super::bitreader::BitReader;

const HEADER_SIZE: usize = 120;
const BLOCK_HEADER_SIZE: usize = 16;

pub struct NcwReader<R: Read + Seek> {
    header: NcwHeader,
    block_offsets: Vec<u32>,
    reader: R,
}

#[derive(Debug)]
struct NcwHeader {
    num_channels: u16,
    bits_per_sample: u16,
    sample_rate: u32,
    num_samples: u32,
    blocks_address_offset: u32,
    blocks_data_offset: u32,
    blocks_data_size: u32,
}

#[derive(Debug)]
struct BlockHeader {
    base_value: i32,
    bits: i16,
    flags: u16,
}

impl<R: Read + Seek> NcwReader<R> {
    pub fn read(mut reader: R) -> Result<Self, Error> {
        let header = NcwHeader::read(&mut reader)?;
        dbg!(&header);

        let block_offsets = Self::read_block_offsets(&mut reader, &header)?;

        Ok(Self {
            header,
            block_offsets,
            reader,
        })
    }

    fn read_block_offsets(reader: &mut R, header: &NcwHeader) -> Result<Vec<u32>, Error> {
        reader.seek(SeekFrom::Start(header.blocks_address_offset as u64))?;

        // Read block headers
        let num_blocks = (header.blocks_data_offset - header.blocks_address_offset) / 4;
        let mut block_offsets = Vec::new();
        for _ in 1..num_blocks {
            block_offsets.push(reader.read_u32_le()?);
        }

        Ok(block_offsets)
    }

    fn decompress_block(
        reader: &mut R,
        block_header: &BlockHeader,
        sample_count: usize,
    ) -> Result<Vec<i32>, Error> {
        let mut samples = Vec::new();

        if block_header.bits == 0 {
            // No compression, read directly
            for _ in 0..sample_count {
                let mut buffer = vec![0u8; 4]; // Assuming 32-bit samples
                reader.read_exact(&mut buffer)?;
                let sample = i32::from_le_bytes(buffer.try_into().unwrap());
                samples.push(sample);
            }
        } else if block_header.bits < 0 {
            // Simple compression
            let bytes_per_sample = block_header.bits.abs() as usize / 8;
            for _ in 0..sample_count {
                let mut buffer = vec![0u8; bytes_per_sample];
                reader.read_exact(&mut buffer)?;
                // Sign-extend to 32 bits
                let mut extended = vec![0u8; 4 - bytes_per_sample];
                extended.extend_from_slice(&buffer);
                let sample = i32::from_le_bytes(extended.try_into().unwrap());
                samples.push(sample as i32);
            }
        } else {
            // Delta compression
            let bits = block_header.bits as usize;

            let mut current_base = block_header.base_value;
            samples.push(current_base as i32);

            // let mut read_buffer = [0; 8]; // Max 64 bits per read
            // reader.read_exact(&mut read_buffer[..((bits + 7) / 8)])?;
            //
            // let mut read_idx = 0;
            // while read_idx < read_buffer.len() * 8 {
            //     let sample = read_buffer[read_idx / 8] as i16;
            //     let delta = sample & ((1 << bits) - 1);
            //     current_base += delta as i32;
            //     samples.push(current_base as i32);
            //     read_idx += bits;
            // }

            let mut bit_reader = BitReader::new(reader);

            for _ in 0..sample_count {
                // sample += *delta as i32;
                // data.push(sample as i16);

                let delta = bit_reader.read_bits_i16(bits)? as i32;
                current_base += delta;
                samples.push(current_base);
            }
        }

        Ok(samples)
    }
    /// Decode all samples
    pub fn decode(&mut self) -> Result<Vec<i32>, Error> {
        let mut samples = Vec::new();
        // Seek to block base address
        let block_address = self.header.blocks_data_offset;
        self.reader.seek(SeekFrom::Start(block_address as u64))?;

        dbg!(self.header.num_blocks());

        for _ in 0..self.header.num_blocks() {
            let block_header = BlockHeader::read(&mut self.reader)?;
            for sample in Self::decompress_block(&mut self.reader, &block_header, 512)? {
                samples.push(sample);
            }
        }

        // let block_section_len = block_header.bits * 64;
        // let bits = block_header.bits as usize;
        //
        // // Read block data
        // let block_data = self.reader.read_bytes(block_section_len as usize)?;
        // assert_eq!(block_data.len(), 1216);
        //
        // let mut read_buffer = [0; 8]; // Max 64 bits per read
        // self.reader
        //     .read_exact(&mut read_buffer[..((bits + 7) / 8)])?;
        //
        // if block_header.bits > 0 {
        //     // DPCM
        //     let deltas = block_data;
        //     let initial_sample = block_header.base_value;
        //     dbg!(deltas.len());
        //
        //     let mut tmp_samples = Vec::with_capacity(512); // vec to store 19-bit deltas
        //     let mut base_sample = initial_sample;
        //     tmp_samples.push(initial_sample);
        //
        //     let mut offset = 0;
        //     while tmp_samples.len() <= 64 {
        //         let mut delta = 0i16;
        //         for _ in 0..19 {
        //             let byte = deltas[offset];
        //             delta = (delta << 8) | (byte as i16);
        //             dbg!(delta, tmp_samples.len());
        //             offset += 1;
        //         }
        //         base_sample += delta as i32;
        //         tmp_samples.push(base_sample);
        //     }
        // } else {
        //     todo!();
        // }
        //
        // // let mut read_idx = 0;
        // // while read_idx < read_buffer.len() * 8 {
        // //     let sample = read_buffer[read_idx / 8];
        // //     let delta = sample & ((1 << bits) - 1);
        // //     current_base += delta as i32;
        // //     samples.push(current_base);
        // //     read_idx += bits;
        // // }
        //
        // // for sample in data {
        // //     let truncated = sample & (2u8.pow(header.bits as u32) - 1);
        // //     samples.push(truncated as i32);
        // // }
        // // }
        // let block_header = BlockHeader::read(&mut self.reader)?;
        // dbg!(block_header);

        Ok(samples)
    }

    // fn read_block<R: Read + Seek>(mut reader: R, block_index: u32) -> Result<Vec<i16>, Error> {
    //     // Seek to block offset address
    //     let block_address = self.header.blocks_address_offset + (block_index * 4) as u32;
    //     self.reader
    //         .seek(SeekFrom::Start(block_address as u64))
    //         .unwrap();
    //
    //     // Read block offset
    //     let mut block_offset = [0; 4];
    //     self.reader.read_exact(&mut block_offset).unwrap();
    //     let block_offset = u32::from_le_bytes(block_offset);
    //
    //     // Seek to block data
    //     let block_data_pos = self.header.blocks_data_offset + block_offset as u32;
    //     self.reader
    //         .seek(SeekFrom::Start(block_data_pos as u64))
    //         .unwrap();
    //
    //     // Read block header
    //     let mut block_header = BlockHeader::read(&mut self.reader)?;
    //     dbg!(&block_header);
    //
    //     // Read block data
    //     let mut block = vec![0i16; 512];
    //     let bits = block_header.bits.abs() as u8;
    //     let _bytes_per_sample = (bits as usize + 7) / 8;
    //
    //     for i in 0..512 {
    //         let mut sample = [0; 2];
    //         self.reader.read_exact(&mut sample).unwrap();
    //         let sample = i16::from_le_bytes(sample);
    //
    //         if block_header.bits < 0 {
    //             // No compression
    //             block[i] = sample;
    //         } else if block_header.bits > 0 {
    //             // DPCM
    //             block[i] = block_header.base_value as i16 + sample;
    //             block_header.base_value = block[i] as i32;
    //         } else {
    //             // Bit truncation
    //             let mask = (1 << bits) - 1;
    //             block[i] = (sample & mask as i16) as i16;
    //         }
    //     }
    //
    //     if block_header.flags == 1 {
    //         // MID/SIDE
    //         let mut left = Vec::with_capacity(512);
    //         let mut right = Vec::with_capacity(512);
    //
    //         for i in 0..512 {
    //             if i % 2 == 0 {
    //                 left.push(block[i]);
    //             } else {
    //                 right.push(block[i]);
    //             }
    //         }
    //
    //         for i in 0..256 {
    //             let mid = (left[i] + right[i]) >> 1;
    //             let side = left[i] - right[i];
    //
    //             block[i * 2] = mid;
    //             block[i * 2 + 1] = side;
    //         }
    //     }
    //
    //     Ok(block)
    // }
}

impl NcwHeader {
    pub fn read<R: ReadBytesExt>(reader: &mut R) -> Result<Self, Error> {
        let mut reader = Cursor::new(reader.read_bytes(HEADER_SIZE)?);

        let magic = reader.read_u64_be()?;
        assert!([0x01A89ED631010000, 0x01A89ED630010000].contains(&magic));

        Ok(Self {
            num_channels: reader.read_u16_le()?,
            bits_per_sample: reader.read_u16_le()?,
            sample_rate: reader.read_u32_le()?,
            num_samples: reader.read_u32_le()?,
            blocks_address_offset: reader.read_u32_le()?,
            blocks_data_offset: reader.read_u32_le()?,
            blocks_data_size: reader.read_u32_le()?,
        })
    }

    pub fn num_blocks(&self) -> u32 {
        let block_offsets_len = self.blocks_data_offset - self.blocks_address_offset;
        block_offsets_len / 4
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
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;

    use super::*;

    #[test]
    fn test_read() -> Result<(), Error> {
        let reader = File::open("test-data/NCW/16-bit.ncw")?;
        let mut ncw = NcwReader::read(reader)?;
        let samples = ncw.decode()?;

        let mut file = File::create("out.pcm")?;
        for sample in samples {
            let bytes = sample.to_le_bytes();
            file.write_all(&bytes)?;
        }

        Ok(())
    }
}
