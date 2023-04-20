use crate::{nisound::AuthoringApplication, prelude::*};

pub struct AppSpecific {
    authoring_app: AuthoringApplication,
    // subtree_item: SubtreeItem,
}

impl AppSpecific {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {
        log::debug!("SubtreeItem::read");

        let prop_version = reader.read_u32_le()?;
        debug_assert_eq!(prop_version, 1);

        let authoring_app: AuthoringApplication = reader.read_u32_le()?.into();
        log::debug!("authoring_app_id: {:?}", authoring_app);

        Ok(AppSpecific { authoring_app })
    }
}
