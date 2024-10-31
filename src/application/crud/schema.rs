use axum_typed_multipart::{FieldData, TryFromMultipart};
use serde::Deserialize;
use tempfile::NamedTempFile;
use garde::Validate;


#[derive(Debug, Validate, TryFromMultipart)]
pub struct NewAssetTypeSchema {
    #[garde(length(min=1))]
    pub brand: String,
    #[garde(length(min=1))]
    pub model: String,
    #[garde(skip)]	
    pub description: Option<String>,
    #[garde(skip)]	
    pub cost: Option<String>,
    #[garde(skip)]	
    #[form_data(limit = "5MiB")]
    pub picture: Option<FieldData<NamedTempFile>>,
}

#[derive(Debug, Validate, TryFromMultipart)]
pub struct UpdateAssetTypeSchema {
    #[garde(length(min=1))]
    pub brand: String,
    #[garde(length(min=1))]
    pub model: String,
    #[garde(skip)]	
    pub description: Option<String>,
    #[garde(skip)]	
    pub cost: Option<String>,
    #[garde(skip)]	
    #[form_data(limit = "5MiB")]
    pub picture: Option<FieldData<NamedTempFile>>,
}

#[derive(Debug, TryFromMultipart)]
pub struct UploadAsetTypesSchema {
    #[form_data(limit = "5MiB")]
    pub upload: FieldData<NamedTempFile>,
}


