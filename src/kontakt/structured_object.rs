use std::fmt::Debug;

use crate::prelude::io;
use crate::{read_bytes::ReadBytesExt, Error, NIFileError};

use super::chunk::Chunk;

#[doc = include_str!("../../doc/presets/Kontakt/StructuredObject.md")]
pub struct StructuredObject {
    pub version: u16,
    pub public_data: Vec<u8>,
    pub private_data: Vec<u8>,
    pub children: Vec<Chunk>,
}

impl Debug for StructuredObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StructuredObject")
            .field("version", &format_args!("0x{:X}", self.version))
            .field("public_data_bytes", &self.public_data.len())
            .field("private_data_bytes", &self.private_data.len())
            .field("child_count", &self.children.len())
            .finish()
    }
}

impl StructuredObject {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let is_data_structured = reader.read_bool()?;
        if !is_data_structured {
            let mut buf = Vec::new();
            // NOTE: I think a u16 for version is still read here...
            reader.read_to_end(&mut buf)?;
            return Ok(Self {
                public_data: buf,
                version: 0,
                private_data: Vec::new(),
                children: Vec::new(),
            });
        }

        let public_data_version = reader.read_u16_le()?;

        let private_data_length = reader.read_u32_le()?;
        let private_data = reader
            .read_bytes(private_data_length as usize)
            .map_err(|e| {
                NIFileError::Generic(format!(
                    "Failed to read StructuredObject private_data: length={private_data_length} error={e}",
                ))
            })?;

        let public_data_length = reader.read_u32_le()?;
        let public_data = reader
            .read_bytes(public_data_length as usize)
            .map_err(|e| {
                NIFileError::Generic(format!(
                    "Failed to read StructuredObject public_data: length={public_data_length} version={public_data_version} error={e}",
                ))
            })?;

        let children_data_length = reader.read_u32_le()?;
        let children_data = reader
            .read_bytes(children_data_length as usize)
            .map_err(|e| {
                NIFileError::Generic(format!(
                    "Failed to read StructuredObject private_data: length={children_data_length} error={e}",
                ))
            })?;
        let mut children_reader = io::Cursor::new(children_data);

        let mut children = Vec::new();
        while let Ok(object) = Chunk::read(&mut children_reader) {
            children.push(object);
        }

        Ok(Self {
            private_data,
            version: public_data_version,
            public_data,
            children,
        })
    }

    pub fn find_first(&self, id: u16) -> Option<&Chunk> {
        self.children.iter().find(|c| c.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_structured_object_0x28() -> Result<(), Error> {
        let mut file = File::open("tests/patchdata/KontaktV42/StructuredObject/0x28")?;
        let obj = StructuredObject::read(&mut file)?;

        assert_eq!(obj.version, 0x80);
        assert_eq!(obj.public_data.len(), 140);
        assert_eq!(obj.private_data.len(), 211);
        assert_eq!(obj.children.len(), 5);

        Ok(())
    }
}
