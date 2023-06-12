use std::io::Read;

use crate::prelude::*;
use crate::read_bytes::ReadBytesExt;

/*

ProgramContainer {
    name: wstring
    volume: f32
    pan: f32
}

ZoneData {
}

*/

// defaultFactory:
// if id < 0xF (15)
//   if id == 5 ( new ? )
//   if id == 6 ( new ? )
// 0x2b (43)

// see dbgPrint, K4PO, K4PL (::read for sizes)

// InternalPatchData::ExtractHeader
//  - finds BNISoundHeader
// - md5 hashsum

// SECTIONS:
//
// K4PO::K4PL_Script (7 params)
//   string
//   bool
//   bool
//   bool
//   string
//   string
//   string
//   u32
//   ?string
//
// Loop (7 params)      loc         25 bytes?
//   mode               0x04 i32
//   loopStart          0x08 i32
//   loopLength         0x0C i32
//   loopCount          0x10 i32
//   alternatingLoop    0x14 bool
//   loopTuning         0x18 f32
//   xfadeLength        0x1C i32
//
// ZoneDataV98 (26 params)
//   sampleStart            0x04 i32
//   sampleEnd              0x08 i32
//   sampleStartModRange    0x0C i32
//   lowVelocity            0x10 i16
//   highVelocity           0x12 i16
//   lowKey                 0x14 i16
//   highKey                0x16 i16
//   fadeLowVelo            0x18 i16
//   fadeHighVelo           0x1A i16
//   fadeLowKey             0x1C i16
//   fadeHighKey            0x1E i16
//   rootKey                0x20 i16
//   zoneVolume             0x24 f32
//   zonePan                0x28 f32
//   zoneTune               0x2C f32
//   fileNameId             0x30 i32
//   sampleDataType         0x34 i32
//   sampleRate             0x38 i32
//   numChannels            0x3C u8
//   numFrames              0x40 i32
//   reserved1              0x44 i32
//   rootNote               0x48 i32
//   tuning                 0x4C i32
//   reserved3              0x50 f32
//   reserved4              0x54 bool
//
// Group (15 params)
//   name                   0x04 wstring
//   volume                 f32
//   pan                    f32
//   tune                   f32
//   keyTracking            bool
//   reverse
//   releaseTrigger
//   releaseTriggerNoteMonophonic
//   rlsTrigCounter
//   midiChannel
//   voiceGroupIdx
//   fxIdxAmpSplitPoint
//   muted
//   soloed
//   interpQuality
//
// Bank (4 params)
//   masterVolume   f32
//   masterTune     f32
//   masterTempo    i32
//   name           wstring
//

#[derive(Debug, Clone)]
pub struct KontaktPreset();

impl KontaktPreset {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {
        log::debug!("PresetChunkItem::read");

        // ChunkData::doRead
        let magic = reader.read_u16_le()?; // 40, 0x28
        assert_eq!(magic, 0x28);
        println!("magic {}", magic);

        let first_data_block_size = reader.read_i32_le()?; // actually i32
        println!("first_data_block_size {}", first_data_block_size);

        // StructuredObject::doRead
        let parse_object: bool = reader.read_u8()? == 1; // if 0, read raw
        println!("parse_object {}", parse_object);

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
        println!("block_1_version {}", block_1_version);
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

        let unknown = metadata_data.read_i32_le()?;
        assert_eq!(unknown, 0);

        let float_1 = metadata_data.read_f32_le()?;
        println!("float_1: {}", float_1);

        // null term
        assert_eq!(metadata_data.read_u8()?, 0);

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
        //crate::utils::setup_logger();

        for path in crate::utils::get_files("tests/data/nisound/preset/kontakt/**/002-single-sample-2")? {
            println!("reading {:?}", path);

            let file = std::fs::File::open(&path)?;
            KontaktPreset::read(file)?;
        }

        Ok(())
    }
}
