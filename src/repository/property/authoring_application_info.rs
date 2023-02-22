use crate::{
    read_bytes::ReadBytesExt,
    repository::{item_frame::app_id::AuthoringApplication, ItemError},
};

pub struct AuthoringApplicationInfo {
    authoring_app: AuthoringApplication,
    authoring_app_version: String,
}

impl AuthoringApplicationInfo {
    pub fn read<R>(mut reader: R) -> Result<AuthoringApplicationInfo, ItemError>
    where
        R: ReadBytesExt,
    {
        let prop_version = reader.read_u32_le()?;
        assert_eq!(prop_version, 1);

        let app_id = reader.read_u32_le()?;
        let authoring_app = AuthoringApplication::from(app_id);

        let authoring_app_version = reader.read_widestring_utf16()?;

        Ok(AuthoringApplicationInfo {
            authoring_app,
            authoring_app_version,
        })
    }
}
