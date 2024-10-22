use axum_typed_multipart::{FieldData, TryFromMultipart};
use tempfile::NamedTempFile;


#[derive(TryFromMultipart)]
pub struct ProfileImageSchema {
    #[form_data(limit = "5MiB")]
    pub image: FieldData<NamedTempFile>,
}

#[derive(TryFromMultipart)]
pub struct SingleUploadSchema {
    #[form_data(limit = "5MiB")]
    pub field: FieldData<NamedTempFile>,
}