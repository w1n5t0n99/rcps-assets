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
    pub brand: String, 
    pub model: String,
    pub description: Option<String>,
    pub cost: Option<String>,
    pub picture: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateAssetType {
    pub brand: String, 
    pub model: String,
    pub description: Option<String>,
    pub cost: Option<String>,
}