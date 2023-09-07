use crate::prelude::*;
use crate::read_bytes::ReadBytesExt;

/// Kontakt archive that bundles a preset, samples and other files.
pub struct NIFileContainer(Vec<u8>);

impl NIFileContainer {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {
        log::debug!("NIFileContainer::read");

        // NI FC MTD
        // Native Instruments FileContainer MetaData
        let mtd_magic = reader.read_bytes(16)?;
        debug_assert_eq!(
            mtd_magic, b"/\\ NI FC MTD  /\\",
            "Monolith header tag not found."
        );

        let _header_chunk = reader.read_bytes(256)?;
        let file_count = reader.read_u64_le()?;
        let _total_size = reader.read_u64_le()?;

        // NI FC TOC
        // Native Instruments FileContainer Table Of Contents
        // Table 1
        let mtd_magic = reader.read_bytes(16)?;
        debug_assert_eq!(mtd_magic, b"/\\ NI FC TOC  /\\");

        let _header_chunk = reader.read_bytes(600)?;

        for _ in 0..file_count {
            let file_index = reader.read_u64_le()?;
            let _ = reader.read_bytes(16)?;
            let filename = reader.read_bytes(600)?;
            let _ = reader.read_u64_le()?;
        }

        Ok(Self(Vec::new()))
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn test_read() -> Result<()> {
        let file = File::open("tests/filetype/monolith/kontakt/000-default.nki")?;
        NIFileContainer::read(file)?;
        Ok(())
    }
}
