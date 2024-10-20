use std::path::PathBuf;

use chrono::{DateTime, Utc};
use mime_guess::Mime;
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Attachment {
    pub id: i32,
    pub filename: String,
    pub hash: String,
    pub content_type: String,
    pub created_at: DateTime<Utc>,
}

