use axum_typed_multipart::{FieldData, TryFromMultipart};
use tempfile::NamedTempFile;
use garde::Validate;


#[derive(Debug, Validate, TryFromMultipart)]
pub struct NewAssetTypeSchema {
    #[garde(length(min=1))]
    pub brand: String,
    #[garde(length(min=1))]
    pub model: String,
    #[garde(skip)]	
    #[form_data(limit = "5MiB")]
    pub picture0: Option<FieldData<NamedTempFile>>,
}

