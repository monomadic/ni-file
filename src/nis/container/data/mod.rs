pub mod item_data_header;
pub mod item_type;

pub use item_data_header::*;
pub use item_type::*;

use crate::{read_bytes::ReadBytesExt, Error};
use std::io::{Cursor, Read};

#[derive(Clone, Debug)]
pub struct ItemData {
    pub header: ItemDataHeader,
    pub inner: Option<Box<ItemData>>,
    pub data: Vec<u8>,
}

impl ItemData {
    pub fn child(&self) -> Option<&ItemData> {
        self.inner.as_ref().map(Box::as_ref)
    }

    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let header = ItemDataHeader::read(&mut reader)?;
        let length = header.length as usize - 20;

        match header.item_type() {
            ItemType::Item => {
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
    fn test_item_frame_read_000() -> Result<(), Error> {
        let file = File::open("tests/patchdata/NISD/ItemFrame/RepositoryRoot-000")?;
        let item = ItemData::read(file)?;

        assert_eq!(item.data.len(), 58);
        assert_eq!(item.header.item_type(), ItemType::RepositoryRoot);
        assert_eq!(
            item.inner.unwrap().header.item_type(),
            ItemType::Authorization
        );

        Ok(())
    }

    #[test]
    fn test_item_frame_read_001() -> Result<(), Error> {
        let file = File::open("tests/patchdata/NISD/ItemFrame/RepositoryRoot-001")?;
        let item = ItemData::read(file)?;

        assert_eq!(item.data.len(), 58);
        assert_eq!(item.header.item_type(), ItemType::RepositoryRoot);
        assert_eq!(
            item.inner.unwrap().header.item_type(),
            ItemType::Authorization
        );

        Ok(())
    }
}
