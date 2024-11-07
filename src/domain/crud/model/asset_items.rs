use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow, sqlx::Type)]
pub struct AssetItemID {
    pub id: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow, sqlx::Type)]
pub struct AssetItem {
    pub id: i32,
    pub asset_id: Option<String>,
    pub serial_number: Option<String>,
    pub name: Option<String>,
    pub brand: Option<String>, 
    pub model: Option<String>,
    pub description: Option<String>,
    pub cost: Option<String>,
    pub school: Option<String>,
    pub room: Option<String>,
    pub funding_source: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewAssetItem {
    #[serde(alias="asset_id", alias="Asset ID")]
    pub asset_id: Option<String>, 
    #[serde(alias="serial_number", alias="Serial #")]
    pub serial_number: Option<String>,
    #[serde(alias="name", alias="Name")]
    pub name: Option<String>,
    #[serde(alias="brand", alias="Brand")]
    pub brand: Option<String>,
    #[serde(alias="model", alias="Model")]
    pub model: Option<String>,
    #[serde(alias="school", alias="School")]
    pub school: Option<String>,
    #[serde(alias="room", alias="Room")]
    pub room: Option<String>,
    #[serde(alias="funding_source", alias="Funding Source")]
    pub funding_source: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateAssetItem {
    pub asset_id: Option<String>, 
    pub serial_number: Option<String>,
    pub name: Option<String>,
    pub brand: Option<String>,
    pub model: Option<String>,
    pub school: Option<String>,
    pub room: Option<String>,
    pub funding_source: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AssetItemFilter {
    pub search: Option<String>,
    pub sort: Option<String>,
    pub order: Option<String>,
}