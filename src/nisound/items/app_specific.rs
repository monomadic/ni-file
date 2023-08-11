use crate::{
    nisound::{item_frame::ItemFrame, AuthoringApplication, ItemID},
    prelude::*,
};

pub struct AppSpecific {
    pub authoring_app: AuthoringApplication,
    pub version: String,
    // subtree_item: SubtreeItem,
}

impl std::convert::TryFrom<&ItemFrame> for AppSpecific {
    type Error = NIFileError;

    fn try_from(frame: &ItemFrame) -> Result<Self> {
        log::debug!("AppSpecific::try_from");
        debug_assert_eq!(frame.header.item_id, ItemID::AppSpecific);
        AppSpecific::read(frame.data.as_slice())
    }
}

impl AppSpecific {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {
        log::debug!("SubtreeItem::read");

        let prop_version = reader.read_u32_le()?;
        debug_assert_eq!(prop_version, 1);

        let authoring_app: AuthoringApplication = reader.read_u32_le()?.into();
        log::debug!("authoring_app_id: {:?}", authoring_app);

        let version = reader.read_widestring_utf16()?;

        Ok(AppSpecific {
            authoring_app,
            version,
        })
    }
}
