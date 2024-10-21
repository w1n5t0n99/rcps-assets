use std::path::PathBuf;

use chrono::{DateTime, Utc};
use mime_guess::Mime;
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Attachment {
    pub filename: String,
    pub hash: String,
    pub content_type: String,
    pub first_seen_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewAttachment {
    pub filename: String,
    pub hash: String,
    pub content_type: String,
}

