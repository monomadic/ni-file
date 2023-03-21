use crate::prelude::*;
use crate::read_bytes::ReadBytesExt;

pub struct NIMonolith(Vec<u8>);

impl NIMonolith {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {
        log::debug!("NIMonolith::read");

        // NI FC MTD
        // Native Instruments FileContainer MetaData
        let header_chunk = reader.read_bytes(256)?;

        // NI FC TOC
        // Native Instruments FileContainer Table Of Contents
        let _unk = reader.read_u64_le()?;

        Ok(Self(Vec::new()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() -> Result<()> {
        let file = include_bytes!("../../tests/files/monolith/kontakt/000-default.nki");
        NIMonolith::read(file.as_slice())?;
        Ok(())
    }
}
