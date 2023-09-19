pub mod domain_id;
pub mod item_data_header;
pub mod item_id;

pub use domain_id::*;
pub use item_data_header::*;
pub use item_id::*;

use crate::{prelude::*, read_bytes::ReadBytesExt};
use std::io::{Cursor, Read};

#[derive(Clone, Debug)]
pub struct ItemData {
    pub header: ItemDataHeader,
    pub inner: Option<Box<ItemData>>,
    pub data: Vec<u8>,
}

impl ItemData {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {
        let header = ItemDataHeader::read(&mut reader)?;
        let length = header.length as usize - 20;

        match header.item_id {
            ItemID::Item => {
                let data = reader.read_bytes(length)?;

                Ok(Self {
                    header,
                    inner: None,
                    data,
                })
            }
            _ => {
                let mut buf = Cursor::new(reader.read_bytes(length)?);
                let inner = ItemData::read(&mut buf)?;
                let mut data = Vec::new();
                buf.read_to_end(&mut data)?;

                Ok(Self {
                    header,
                    inner: Some(Box::new(inner)),
                    data,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn test_item_frame_read_000() -> Result<()> {
        let file = File::open("tests/patchdata/NISD/ItemFrame/RepositoryRoot-000")?;
        let item = ItemData::read(file)?;

        assert_eq!(item.data.len(), 58);
        assert_eq!(item.header.item_id, ItemID::RepositoryRoot);
        assert_eq!(item.inner.unwrap().header.item_id, ItemID::Authorization);

        Ok(())
    }

    #[test]
    fn test_item_frame_read_001() -> Result<()> {
        let file = File::open("tests/patchdata/NISD/ItemFrame/RepositoryRoot-001")?;
        let item = ItemData::read(file)?;

        assert_eq!(item.data.len(), 58);
        assert_eq!(item.header.item_id, ItemID::RepositoryRoot);
        assert_eq!(item.inner.unwrap().header.item_id, ItemID::Authorization);

        Ok(())
    }
}
