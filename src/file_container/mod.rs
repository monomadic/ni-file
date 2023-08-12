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
        let _file_count = reader.read_u64_le()?;
        let _total_size = reader.read_u64_le()?;

        // NI FC TOC
        // Native Instruments FileContainer Table Of Contents
        let mtd_magic = reader.read_bytes(16)?;
        debug_assert_eq!(mtd_magic, b"/\\ NI FC TOC  /\\");

        Ok(Self(Vec::new()))
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_read() -> Result<()> {
        let file = Cursor::new(include_bytes!(
            "../../tests/filetype/monolith/kontakt/000-default.nki"
        ));
        NIFileContainer::read(file)?;
        Ok(())
    }
}
