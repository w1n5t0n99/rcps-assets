use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow, sqlx::Type)]
pub struct AssetType {
    pub id: i32,
    pub brand: String, 
    pub model: String,
    pub description: Option<String>,
    pub cost: Option<String>,
    pub picture: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewAssetType {
    #[serde(alias="brand", alias="Brand")]
    //TODO: use #[serde(rename_all = "...")]
    pub brand: String, 
    #[serde(alias="model", alias="Model")]
    pub model: String,
    #[serde(alias="description", alias="Description")]
    pub description: Option<String>,
    #[serde(alias="cost", alias="Cost")]
    pub cost: Option<String>,
    #[serde(alias="picture", alias="Picture")]
    pub picture: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateAssetType {
    pub brand: String, 
    pub model: String,
    pub description: Option<String>,
    pub cost: Option<String>,
    pub picture: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UploadResult {
    pub total: usize,
    pub processed: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AssetTypeFilter {
    pub search: Option<String>,
    pub sort: Option<String>,
    pub order: Option<String>,
}
