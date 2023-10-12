// properties:
// - num-hidden-items

use std::io::Cursor;

use crate::{
    nis::{ItemData, ItemType, SubtreeItem},
    prelude::*,
};

use super::preset::AuthoringApplication;

#[derive(Debug)]
pub struct AppSpecific {
    pub subtree_item: SubtreeItem,
    pub authoring_app: AuthoringApplication,
    pub version: String,
}

impl std::convert::TryFrom<&ItemData> for AppSpecific {
    type Error = NIFileError;

    fn try_from(item: &ItemData) -> Result<Self> {
        if item.header.item_type() != ItemType::AppSpecific {
            return Err(NIFileError::ItemWrapError {
                expected: ItemType::AppSpecific,
                got: item.header.item_type(),
            });
        }

        let subtree_item = SubtreeItem::read(&mut Cursor::new(&item.child().unwrap().data))?;

        let mut reader = Cursor::new(&item.data);

        let prop_version = reader.read_u32_le()?;
        debug_assert_eq!(prop_version, 1);

        let authoring_app: AuthoringApplication = reader.read_u32_le()?.into();
        let version = reader.read_widestring_utf16()?;

        Ok(AppSpecific {
            subtree_item,
            authoring_app,
            version,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use super::*;

    // #[test]
    // fn test_app_specific_read() -> Result<()> {
    //     let mut file = File::open("tests/data/Containers/NIS/objects/AppSpecific/AppSpecific-000")?;
    //     let item = AppSpecific::read(&mut file)?;
    //
    //     assert_eq!(item.authoring_app, AuthoringApplication::Kontakt);
    //     assert_eq!(item.version, String::from("7.1.3.0"));
    //
    //     // ensure the read completed
    //     let mut buf = Vec::new();
    //     file.read_to_end(&mut buf)?;
    //     assert_eq!(buf.len(), 0, "Excess data found");
    //
    //     Ok(())
    // }
}
