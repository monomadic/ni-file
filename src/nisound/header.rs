use crate::prelude::*;
use crate::read_bytes::ReadBytesExt;

/// The header chunk of an [`Item`](crate::nisound::Item).
///
/// | Offset | Length | Type      | Meaning                     | Default    | Other                                    |
/// |--------|--------|-----------|-----------------------------|------------|------------------------------------------|
/// | 0      | 8      | uint64_t  | size                        |  |                                          |
/// | 8      | 4      | uint32_t  | version                     | 1 |                                          |
/// | 12     | 4      | uint32_t  | domainID                     |  |                                          |
/// | 16     | 4      | uint32_t  | headerFlags                     |  |                                          |
/// | 20     | 4      | ItemUuid* | itemUuid                     |  |                                          |
///
#[derive(Debug, Clone)]
pub struct ItemHeader {
    /// Size in bytes of the entire [`Item`](super::Item).
    pub size: u64,
    /// Integer that resolves to a [`DomainID`](super::DomainID).
    pub magic: String, // (+0xC, uint, 'hsin')
    pub header_flags: u32, // (0x10, uint)
    pub uuid: Vec<u8>,     // (0x14, 16 bytes, randomly generated)
}

impl ItemHeader {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {
        log::debug!("ItemHeader::read");
        let size = reader.read_u64_le()?;
        // always 1
        let _version = reader.read_u32_le()?;
        let domain_id = reader.read_u32_le()?;
        // if domaind_id = 'hsin'
        if domain_id != 1852404584 {
            return Err(NIFileError::Generic(
                "hsin not found while reading header".into(),
            ));
        };

        let magic = reader.read_u32_le()?.to_string();
        let header_flags = reader.read_u32_le()?;
        // research
        let _unknown = reader.read_u32_le()?;
        let uuid = reader.read_bytes(16)?;

        Ok(Self {
            size,
            magic,
            header_flags,
            uuid,
        })
    }
}
