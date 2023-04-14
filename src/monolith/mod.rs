use crate::prelude::*;
use crate::read_bytes::ReadBytesExt;

/// Kontakt archive that bundles a preset, samples and other files.
pub struct NIMonolith(Vec<u8>);

impl NIMonolith {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {
        log::debug!("NIMonolith::read");

        // NI FC MTD
        // Native Instruments FileContainer MetaData
        let mtd_magic = reader.read_bytes(16)?;
        debug_assert_eq!(mtd_magic, b"/\\ NI FC MTD  /\\");

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
    use super::*;

    #[test]
    fn test_read() -> Result<()> {
        let file = include_bytes!("../../tests/data/monolith/kontakt/000-default.nki");
        NIMonolith::read(file.as_slice())?;
        Ok(())
    }
}
