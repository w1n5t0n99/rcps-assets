use std::path::PathBuf;
use std::collections::HashMap;
use std::sync::LazyLock;

use mime_guess::Mime;


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
    data: Vec<u8>,
    filename: String,
    hash: blake3::Hash,
    tmp_path: PathBuf,
    mime: Mime,
}

pub struct Attachment {

}

/*
    UPLOAD file
    - save field as temp file
    - deduplicate or insert as attachment
        - generate unique file name
        - hash file data
        - save file to somewhere permanent like S3
        - save file atachment info to database
        - use GET //fs/:id route to get uploaded data

        - if file is found in db by unique filehash, return that
    - return attachment data(path, filename, mime)

    - return response (HTMX img tag)
*/