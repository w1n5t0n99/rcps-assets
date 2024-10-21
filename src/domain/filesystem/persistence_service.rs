use std::future::Future;
use std::collections::HashMap;
use std::sync::LazyLock;

use axum_typed_multipart::FieldData;
use tempfile::NamedTempFile;
use thiserror::Error;


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

#[derive(Error, Debug)]
pub enum PersistenceError {
    #[error("file type not supported")]
    ExtNotSupported,
    #[error(transparent)]
    ProcessingFailed(#[from] anyhow::Error),
}

#[derive(Clone, Debug)]

pub struct PersistenceSuccess {
    pub filename: String,
    pub hash: String,
    pub content_type: String,
}

#[derive(Clone, Debug)]
pub struct FilePayload {
    pub data: Vec<u8>,
    pub filename: String,
    pub hash: String,
    pub content_type: String,
}

pub trait PersistenceService {
    //fn persist_field_as_attachment(&self, field: FieldData<NamedTempFile>) -> impl Future<Output = Result<Attachment, PersistenceError>> + Send; 
    fn persist_file(&self, data: Vec<u8>, content_type: String, filename: String) -> impl Future<Output = Result<PersistenceSuccess, PersistenceError>> + Send; 
    fn get_file(&self, hash: String, filename: String) -> impl Future<Output = Result<FilePayload, PersistenceError>> + Send; 
    //fn get_attachent_hash(&self, hash: String)-> impl Future<Output = Result<Attachment, PersistenceError>> + Send;
}