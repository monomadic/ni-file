use crate::prelude::*;
use crate::read_bytes::ReadBytesExt;
use crate::string_reader::StringReader;

const FC_TOC_MARKER_END: u64 = 0xF1F1F1F1F1F1F1F1;
const FC_MTD_MARKER_START: &[u8; 16] = b"/\\ NI FC MTD  /\\";

/// Kontakt archive that bundles a preset, samples and other files.
pub struct NIFileContainer {
    pub file_section_offset: u64,
    pub items: Vec<FileContainerItem>,
}

pub struct FileContainerItem {
    pub index: u64,
    pub filename: String,
    pub file_start_offset: u64,
    pub file_size: u64,
}

impl NIFileContainer {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {
        // NI FC MTD
        // Native Instruments FileContainer MetaData
        let mtd_magic = reader.read_bytes(16)?;
        debug_assert_eq!(
            mtd_magic, FC_MTD_MARKER_START,
            "Monolith header tag not found."
        );

        let _header_chunk = reader.read_bytes(256)?;
        let file_count = reader.read_u64_le()?;
        let total_size = reader.read_u64_le()?;
        dbg!(total_size);

        // NI FC TOC
        // Native Instruments FileContainer Table Of Contents
        // Table 1
        let mtd_magic = reader.read_bytes(16)?;
        debug_assert_eq!(mtd_magic, b"/\\ NI FC TOC  /\\");

        let _header_chunk = reader.read_bytes(600)?;

        let mut offset: u64 = 0;
        let mut items = Vec::new();
        for _ in 0..file_count {
            let index = reader.read_u64_le()?;
            let _ = reader.read_bytes(16)?;

            let buf = reader.read_bytes(600)?;
            let filename = StringReader::read_nullterminated_utf16(&mut io::Cursor::new(buf))?;

            let _ = reader.read_u64_le()?;

            let file_start_offset = offset;
            let file_end_offset = reader.read_u64_le()?;
            let file_size = file_end_offset - file_start_offset;
            offset = file_end_offset;

            items.push(FileContainerItem {
                index,
                filename,
                file_start_offset,
                file_size,
            });
        }

        let end_marker = reader.read_u64_le()?;
        assert_eq!(end_marker, FC_TOC_MARKER_END);

        let _pad = reader.read_bytes(16)?;

        // NI FC TOC
        // Native Instruments FileContainer Table Of Contents
        // Table 2
        let mtd_magic = reader.read_bytes(16)?;
        debug_assert_eq!(mtd_magic, b"/\\ NI FC TOC  /\\");

        let _header_chunk = reader.read_bytes(592)?;

        let file_section_offset = reader.stream_position()?;

        Ok(Self {
            file_section_offset,
            items,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn test_filecontainer_nki() -> Result<()> {
        let file = File::open("tests/data/Containers/FileContainer/files/000-default.nki")?;
        NIFileContainer::read(file)?;
        Ok(())
    }

    #[test]
    fn test_filecontainer_nkm() -> Result<()> {
        let file = File::open("tests/data/Containers/FileContainer/files/001-multi.nkm")?;
        NIFileContainer::read(file)?;
        Ok(())
    }
}
