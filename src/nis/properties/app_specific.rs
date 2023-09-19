// properties:
// - num-hidden-items

use std::io::Cursor;

use crate::{
    nis::{ItemData, ItemID},
    prelude::*,
};

use super::preset::AuthoringApplication;

pub struct AppSpecific {
    pub authoring_app: AuthoringApplication,
    pub version: String,
}

impl std::convert::TryFrom<&ItemData> for AppSpecific {
    type Error = NIFileError;

    fn try_from(frame: &ItemData) -> Result<Self> {
        debug_assert_eq!(frame.header.item_id, ItemID::AppSpecific);
        AppSpecific::read(Cursor::new(&frame.data))
    }
}

impl AppSpecific {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {
        let prop_version = reader.read_u32_le()?;
        debug_assert_eq!(prop_version, 1);

        let authoring_app: AuthoringApplication = reader.read_u32_le()?.into();
        let version = reader.read_widestring_utf16()?;

        Ok(AppSpecific {
            authoring_app,
            version,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn test_app_specific_read() -> Result<()> {
        let file = File::open("test-data/NIS/properties/AppSpecific/AppSpecific-000")?;
        let item = AppSpecific::read(file)?;

        assert_eq!(item.authoring_app, AuthoringApplication::Kontakt);
        assert_eq!(item.version, String::from("7.1.3.0"));
        Ok(())
    }
}
