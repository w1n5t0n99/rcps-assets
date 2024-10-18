use std::path::PathBuf;
use std::collections::HashMap;
use std::sync::LazyLock;

use chrono::{DateTime, Utc};
use mime_guess::Mime;
use serde::{Deserialize, Serialize};


/// Route for passing local assets through the webserver.
/// /content/9e0834c0d3dd1f6a775b9af7523eff7b35e750afb8fcd2753eef06735e13c46f/whatever.jpg

pub static MIME_LOOKUP: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let map: HashMap<&'static str, &'static str> = HashMap::from([
        ("application/json", "json"),
        ("application/pdf", "pdf"),
        ("application/vnd.rn-realmedia", "rm"),
        ("application/x-sh", "sh"),
        ("application/zip", "zip"),
        ("audio/aac", "aac"),
        ("audio/flac", "flac"),
        ("audio/m4a", "m4a"),
        ("audio/mp4", "mp4"),
        ("audio/mpeg", "mp3"),
        ("audio/ogg", "ogg"),
        ("audio/webm", "weba"),
        ("audio/x-matroska", "mka"),
        ("image/apng", "apng"),
        ("image/avif", "avif"),
        ("image/bmp", "bmp"),
        ("image/gif", "gif"),
        ("image/jpeg", "jpeg"),
        ("image/ktx", "ktx"),
        ("image/png", "png"),
        ("image/svg+xml", "svg"),
        ("image/vnd.djvu", "djvu"),
        ("image/webp", "webp"),
        ("image/x-icon", "ico"),
        ("text/html", "html"),
        ("text/plain", "txt"),
        ("text/xml", "xml"),
        ("text/csv", "csv"),
        ("video/mp4", "mp4"),
        ("video/ogg", "ogv"),
        ("video/quicktime", "mov"),
        ("video/webm", "webm"),
        ("video/x-matroska", "mkv"),
        ("video/x-msvideo", "avi"),
    ]);

    map
});

pub struct FilePayload {
    pub data: Vec<u8>,
    pub filename: String,
    pub hash: String,
    pub content_type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Attachment {
    pub id: i32,
    pub filename: String,
    pub hash: String,
    pub content_type: String,
    pub created_at: DateTime<Utc>,
}

/*
    UPLOAD file
    - FilesystemService.generate_payload()

    - deduplicate
    - FilesystemRepo.get_attachment_by_hash()

    - or -

    - FilesystemService.process_payload() // e.g. if image convert to webp and upload to S3 or persist to disk
    - FilesystemRepo.insert_payload_as_attachment() // return hash as id to use in url

    GET file
    - FilesystemRepo.get_attachment_by_hash()
    - FilesystemService.get_payload_by_id()

    - use attachment and payload to generate response with streaming body
*/