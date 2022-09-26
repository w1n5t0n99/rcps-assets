use chrono::prelude::*;

#[derive(Debug, PartialEq, sqlx::FromRow)]
pub struct UploadStatus {
    pub sid: i32,
    pub uploaded_file: String,
    pub upload_date: DateTime<Utc>,
    pub total: i32,
    pub skipped: i32,
    pub added: i32,
}