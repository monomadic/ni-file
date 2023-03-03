use crate::prelude::*;
use crate::read_bytes::ReadBytesExt;

// pub struct ItemHeader(Vec<u8>);

#[derive(Debug, Clone)]
pub struct ItemHeader {
    pub size: u64,
    pub domain_id: u32,    // (+0xC, uint, 'hsin')
    pub header_flags: u32, // (0x10, uint)
    pub uuid: Vec<u8>,     // (0x14, 16 bytes, randomly generated)
}

impl ItemHeader {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {
        let size = reader.read_u64_le()?;
        // always 1
        let _unknown = reader.read_u32_le()?;
        let domain_id = reader.read_u32_le()?;
        // if domaind_id = 'hsin'
        if domain_id != 1852404584 {
            return Err(NIFileError::Generic(
                "hsin not found while reading header".into(),
            ));
        };

        let header_flags = reader.read_u32_le()?;
        // research
        let _unknown = reader.read_u32_le()?;
        let uuid = reader.read_bytes(16)?;

        Ok(Self {
            size,
            domain_id,
            header_flags,
            uuid,
        })
    }
}
