use std::io::Read;

use crate::prelude::*;
use crate::read_bytes::ReadBytesExt;

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

        // ID
        let header_chunk_id = reader.read_u16_le()?;
        println!("header_chunk_id {}", header_chunk_id);
        // known ids
        assert!(vec![165_u16, 172, 175].contains(&header_chunk_id));

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

        let chunk_size = reader.read_i32_le()? as usize;
        let data = reader.read_bytes(chunk_size)?;

        // metadata chunk?
        let chunk_size = reader.read_i32_le()?;
        let data = reader.read_bytes(chunk_size as usize)?;

        let data = data.clone();
        let mut data = data.as_slice();

        let name = data.read_widestring_utf16()?;
        println!("name: {}", name);

        let unknown = data.read_bytes(44)?;

        let icon = data.read_u32_le()?;
        println!("icon: {}", icon);

        let desc = data.read_widestring_utf16()?;
        println!("desc: {}", desc);

        let author = data.read_widestring_utf16()?;
        println!("author: {}", author);

        let weblink = data.read_widestring_utf16()?;
        println!("weblink: {}", weblink);

        let mut buf = Vec::new();
        let _ = data.read_to_end(&mut buf)?;
        println!("remaining data({}): {:x?}", buf.len(), buf);

        // BLOCK 2

        println!("reading block 2");

        // patch chunk?
        let chunk_size = reader.read_i32_le()? as usize;
        let data = reader.read_bytes(chunk_size)?;
        //std::fs::write("patch-data", &data)?;

        let data = data.clone();
        let mut data = data.as_slice();

        let id = data.read_u16_le()?;
        assert_eq!(id, 58);
        // let some_data = data.read_sized_data()?;

        // end patch chunk

        // seems always 71... id?
        let _u = reader.read_u16_le()?;

        // unknown chunk
        let chunk_size = reader.read_i32_le()?;
        let _data = reader.read_bytes(chunk_size as usize)?;

        // seems always 75... id?
        let _u = reader.read_u16_le()?;

        // footer chunk?
        let chunk_size = reader.read_i32_le()?;
        let _data = reader.read_bytes(chunk_size as usize)?;

        Ok(Self {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kontakt_preset_read() -> Result<()> {
        //crate::utils::setup_logger();

        for path in crate::utils::get_files("tests/data/nisound/preset/kontakt/**/*")? {
            println!("reading {:?}", path);

            let file = std::fs::File::open(&path)?;
            KontaktPreset::read(file)?;
        }

        Ok(())
    }
}
