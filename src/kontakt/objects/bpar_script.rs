use std::{fs::write, io::Cursor};

use crate::{
    kontakt::{error::KontaktError, structured_object::StructuredObject, Chunk},
    read_bytes::ReadBytesExt,
    Error,
};

const CHUNK_ID: u16 = 0x06;

/// Type:           Chunk<Data>
/// SerType:        0x06
/// Versions:       0x50, 0x60
/// Kontakt 7:      BParScript
/// KontaktIO:
#[derive(Debug)]
pub struct BParScript(pub StructuredObject);

#[derive(Debug)]
pub struct BParScriptParams {
    text: Option<String>,
    source_editor_open: bool,
    touched_but_not_applied: bool,
    bypass: bool,
    password_hash: String,
    description: Option<String>,
    textfile_name: Option<String>,
}

impl BParScript {
    pub fn params(&self) -> Result<BParScriptParams, Error> {
        write("ob", &self.0.public_data)?;
        let mut reader = Cursor::new(&self.0.public_data);

        let _version = reader.read_u16_le()?;

        Ok(BParScriptParams {
            text: reader.read_optional_sized_utf8()?,
            source_editor_open: reader.read_bool()?,
            touched_but_not_applied: reader.read_bool()?,
            bypass: reader.read_bool()?,
            password_hash: reader.read_sized_utf8()?,
            description: reader.read_optional_sized_utf8()?,
            textfile_name: reader.read_optional_sized_utf8()?,
        })
    }
}

impl std::convert::TryFrom<&Chunk> for BParScript {
    type Error = Error;

    fn try_from(chunk: &Chunk) -> Result<Self, Self::Error> {
        if chunk.id != CHUNK_ID {
            return Err(KontaktError::IncorrectID {
                expected: CHUNK_ID,
                got: chunk.id,
            }
            .into());
        }
        Ok(Self(chunk.try_into()?))
    }
}
