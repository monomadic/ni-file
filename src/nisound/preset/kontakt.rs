use std::io::Read;

use crate::prelude::*;
use crate::read_bytes::ReadBytesExt;

fn read_chunks<R: ReadBytesExt>(mut reader: R) -> Result<Vec<(u16, Vec<u8>)>> {
    // for now lets read the entire thing to end
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let mut buf = buf.as_slice();

    let mut chunks = Vec::new();

    while !buf.is_empty() {
        let id = buf.read_u16_le()?;
        let len = buf.read_u32_le()?;
        let data = buf.read_bytes(len as usize)?;
        chunks.push((id, data));
    }

    Ok(chunks)
}

#[derive(Debug, Clone)]
pub struct KontaktPreset();

impl KontaktPreset {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {
        log::debug!("PresetChunkItem::read");

        // ChunkData::doRead
        let magic = reader.read_u16_le()?; // 40, 0x28
        assert_eq!(magic, 0x28);
        println!("magic 0x{:x}", magic);

        let first_data_block_size = reader.read_i32_le()?; // actually i32
        println!("first_data_block_size {}", first_data_block_size);

        // StructuredObject::doRead
        let read_skipped: bool = reader.read_u8()? == 1; // if 0, read raw
        println!("read_skipped {}", read_skipped);

        // BLOCK 1
        // metadata
        // Versions of block 1 chunks:
        //
        // 165 kontakt 5
        //     97 bytes
        //
        // 172 kontakt 6
        //     103 bytes
        //
        // 175 kontakt 7
        //     113 bytes

        let block_1_version = reader.read_u16_le()?;
        println!("block_1_version 0x{:X}", block_1_version);
        // known versions: 165, 172, 175
        assert!(vec![165_u16, 172, 175].contains(&block_1_version));

        let block_1_size = reader.read_i32_le()? as usize;
        println!("block_1_size {}", block_1_size);

        let block_1_data = reader.read_bytes(block_1_size)?;
        assert_eq!(block_1_data.len(), block_1_size);
        println!("block_1_data: {:?}", &block_1_data);

        let mut block_1_data = block_1_data.as_slice();

        // // block 1: unknown data chunk
        //
        // let block_1_unknown_length = block_1_data.read_i32_le()? as usize;
        // println!("block_1_unknown_length: {}", block_1_unknown_length);
        // let block_1_unknown_data = block_1_data.read_bytes(block_1_unknown_length)?;
        // assert_eq!(block_1_unknown_data.len(), block_1_unknown_length);

        // block 1: metadata chunk
        println!("reading metadata_chunk");

        let metadata_length = reader.read_i32_le()? as usize;
        println!("metadata_length: {}", metadata_length);

        let metadata_data = reader.read_bytes(metadata_length)?;
        let mut metadata_data = metadata_data.as_slice();

        let name = metadata_data.read_widestring_utf16()?;
        println!("name: {}", name);

        let num_bytes_samples_total = metadata_data.read_f64_le()?;
        println!("num_bytes_samples_total: {}", num_bytes_samples_total);

        let transpose = metadata_data.read_u8()?;
        println!("transpose: {}", transpose);

        // null term
        // assert_eq!(metadata_data.read_u8()?, 0);

        let master_volume = metadata_data.read_f32_le()?;
        println!("master_volume: {}", master_volume);

        let master_pan = metadata_data.read_f32_le()?;
        println!("master_pan: {}", master_pan); // 1.0 = 100R, -1.0 = 100L

        // null term
        assert_eq!(metadata_data.read_u8()?, 0);

        let master_tuning = metadata_data.read_f32_le()?;
        println!("master_tuning: {:?}", master_tuning);

        let unknown = metadata_data.read_bytes(22)?;
        println!("unknown: {:?}", unknown);

        let icon = metadata_data.read_u32_le()?;
        println!("icon: {}", icon);

        let desc = metadata_data.read_widestring_utf16()?;
        println!("desc: {}", desc);

        let author = metadata_data.read_widestring_utf16()?;
        println!("author: {}", author);

        let weblink = metadata_data.read_widestring_utf16()?;
        println!("weblink: {}", weblink);

        let mut buf = Vec::new();
        let _ = metadata_data.read_to_end(&mut buf)?;
        println!("remaining data({}): {:x?}", buf.len(), buf);

        // INSTRUMENT CHUNK
        println!("reading instrument data");

        // patch chunk?
        let block_2_length = reader.read_i32_le()? as usize;
        println!("block_2_length {}", &block_2_length);
        let instrument_data = reader.read_bytes(block_2_length)?;
        //std::fs::write("patch-data", &data)?;

        println!("**");

        Ok(Self {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kontakt_preset_read() -> Result<()> {
        for path in crate::utils::get_files("tests/data/nisound/preset/kontakt/**/*")? {
            println!("reading {:?}", path);

            let file = std::fs::File::open(&path)?;
//            KontaktPreset::read(file)?;
            let chunks = read_chunks(&file)?;

            // top level chunks
            println!("{:?}", chunks.iter()
                     .map(|c| format!("0x{:x}-{}", c.0, c.1.len()))
                     .collect::<Vec<String>>().join(","));
        }

        Ok(())
    }
}
