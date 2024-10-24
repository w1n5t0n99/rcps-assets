use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use compact_str::CompactString;
use derive_more::derive::{Display, AsRef};


#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow, sqlx::Type)]
pub struct AssetType {
    pub id: i32,
    pub brand: String, 
    pub model: String,
    pub picture0: Option<String>,
    pub picture1: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewAssetType {
    pub brand: String, 
    pub model: String,
    pub picture0: Option<String>,
    pub picture1: Option<String>,
}