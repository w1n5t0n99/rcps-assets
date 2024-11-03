use axum_typed_multipart::{FieldData, TryFromMultipart};
use serde::{Deserialize, Serialize};
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

#[derive(Debug, Validate, TryFromMultipart)]
pub struct UploadAsetTypesSchema {
    #[garde(skip)]	
    #[form_data(limit = "5MiB")]
    pub upload: FieldData<NamedTempFile>,
    #[garde(custom(is_valid_method))]
    pub method: String,
}

fn is_valid_method(value: &str, _: &()) -> garde::Result {
    match value {
        "add" => Ok(()),
        "add_or_update" => Ok(()),
        _ => Err(garde::Error::new("invalid upload method")),
    }
}

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct AssetTypeFilterSchema {
    #[garde(skip)]	
    pub search: Option<String>,
    #[garde(custom(validate_sort))]
    pub sort: Option<String>,
    #[garde(custom(validate_order))]
    pub order: Option<String>,
}

fn validate_order(value: &Option<String>, _: &()) -> garde::Result {
    if let Some(order) = value {
        match order.to_uppercase().as_str() {
            "ASC" => garde::Result::Ok(()),
            "DESC" => garde::Result::Ok(()),
            "" => garde::Result::Ok(()),
            _ => garde::Result::Err(garde::Error::new("invalid order type")),
        }
    } else {
        garde::Result::Ok(())
    }
}

fn validate_sort(value: &Option<String>, _: &()) -> garde::Result {
    if let Some(order) = value {
        match order.as_str() {
            "brand" => garde::Result::Ok(()),
            "model" => garde::Result::Ok(()),
            "" => garde::Result::Ok(()),
            _ => garde::Result::Err(garde::Error::new("invalid sort type")),
        }
    } else {
        garde::Result::Ok(())
    }
}


