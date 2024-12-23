use axum_typed_multipart::{FieldData, TryFromMultipart};
use tempfile::NamedTempFile;


#[derive(TryFromMultipart)]
pub struct ImageSchema {
    #[form_data(limit = "5MiB")]
    pub image: FieldData<NamedTempFile>,
}
