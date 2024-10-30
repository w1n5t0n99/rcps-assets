use std::{collections::HashMap, ffi::OsStr, path::PathBuf, sync::LazyLock};

use chrono::{DateTime, Utc};
use mime_guess::Mime;
use serde::{Deserialize, Serialize};
use derive_more::derive::{Display, AsRef};
use tempfile::NamedTempFile;


/// Route for passing local assets through the webserver.
/// /content/9e0834c0d3dd1f6a775b9af7523eff7b35e750afb8fcd2753eef06735e13c46f/whatever.jpg
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ExtensionType {
    Image,
    Text,
    Application,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Extension {
    PDF,
    JPEG,
    GIF,
    PNG,
    BMP,
    ICO,
    WEBP,
    SVG,
    CSV,
    TXT,
}

impl Extension {
    pub fn ext_type(&self) -> ExtensionType {
        match self {
            Extension::PDF => ExtensionType::Application,
            Extension::JPEG => ExtensionType::Image,
            Extension::GIF => ExtensionType::Image,
            Extension::PNG => ExtensionType::Image,
            Extension::BMP => ExtensionType::Image,
            Extension::ICO => ExtensionType::Image,
            Extension::WEBP => ExtensionType::Image,
            Extension::SVG => ExtensionType::Image,
            Extension::CSV => ExtensionType::Text,
            Extension::TXT => ExtensionType::Text,
        }
    }
}

impl std::fmt::Display for Extension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ext = match self {
            Extension::PDF => "pdf",
            Extension::JPEG => "jpeg",
            Extension::GIF => "gif",
            Extension::PNG => "png",
            Extension::BMP => "bmp",
            Extension::ICO => "ico",
            Extension::WEBP => "webp",
            Extension::SVG => "svg",
            Extension::CSV => "csv",
            Extension::TXT => "txt",
        };

        write!(f, "{}", ext)
    }
}

pub static MIME_LOOKUP: LazyLock<HashMap<&'static str, Extension>> = LazyLock::new(|| {
    let map: HashMap<&'static str, Extension> = HashMap::from([
       // ("application/json", "json"),
        ("application/pdf", Extension::PDF),
       // ("application/vnd.rn-realmedia", "rm"),
       // ("application/x-sh", "sh"),
       // ("application/zip", "zip"),
       // ("audio/aac", "aac"),
       // ("audio/flac", "flac"),
       // ("audio/m4a", "m4a"),
       // ("audio/mp4", "mp4"),
       // ("audio/mpeg", "mp3"),
       // ("audio/ogg", "ogg"),
       // ("audio/webm", "weba"),
       // ("audio/x-matroska", "mka"),
       // ("image/apng", "apng"),
       // ("image/avif", "avif"),
        ("image/bmp", Extension::BMP),
        ("image/gif", Extension::GIF),
        ("image/jpeg", Extension::JPEG),
       // ("image/ktx", "ktx"),
        ("image/png", Extension::PNG),
        ("image/svg+xml", Extension::SVG),
       // ("image/vnd.djvu", "djvu"),
        ("image/webp", Extension::WEBP),
        ("image/x-icon", Extension::ICO),
       // ("text/html", "html"),
        ("text/plain", Extension::TXT),
       // ("text/xml", "xml"),
        ("text/csv", Extension::CSV),
       // ("video/mp4", "mp4"),
       // ("video/ogg", "ogv"),
       // ("video/quicktime", "mov"),
       // ("video/webm", "webm"),
       // ("video/x-matroska", "mkv"),
       // ("video/x-msvideo", "avi"),
    ]);

    map
});

#[derive(Debug)]
pub struct FilePayload {
    pub temp_file: NamedTempFile,
    pub filename: String,
    pub hash: String,
    pub content_type: String,
}

#[derive(Clone, Debug, Display, Serialize, Deserialize, AsRef, sqlx::Type)]
#[as_ref(str, [u8], String)]
pub struct ContentType(String);

impl ContentType {
    pub fn new(raw_value: impl Into<String>) -> Self {
        Self(raw_value.into())
    }
}

#[derive(Clone, Debug, Display, Serialize, Deserialize, AsRef, sqlx::Type)]
#[as_ref(str, [u8], String)]
pub struct Filename(String);

impl Filename {
    pub fn new(raw_value: impl Into<String>) -> Self {
        Self(raw_value.into())
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct NewImageAttachment {
    pub filename: Filename,
    pub hash: String,
    pub content_type: ContentType,
    pub url: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct NewDocumentAttachment {
    pub filename: Filename,
    pub hash: String,
    pub content_type: ContentType,
    pub url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow, sqlx::Type)]
pub struct ImageAttachment {
    pub id: i32,
    pub filename: Filename,
    pub hash: String,
    pub content_type: ContentType,
    pub url: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow, sqlx::Type)]
pub struct DocumentAttachment {
    pub id: i32,
    pub filename: Filename,
    pub hash: String,
    pub content_type: ContentType,
    pub url: String,
    pub created_at: DateTime<Utc>,
}