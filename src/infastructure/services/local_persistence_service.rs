use anyhow::{anyhow, Context};
use image::EncodableLayout;
use tempfile::NamedTempFile;
use std::{io::Write, path::{Path, PathBuf}};

use crate::domain::filesystem::persistence_service::{FilePayload, PersistenceError, PersistenceService, PersistenceSuccess};


#[derive(Debug, Clone)]
pub struct LocalPersistenceService {
    content_directory: PathBuf
}

impl LocalPersistenceService {
    pub fn new(content_directory: impl AsRef<Path>) -> anyhow::Result<Self> {
        if content_directory.as_ref().is_dir() == true {
            return Ok(Self {content_directory: content_directory.as_ref().into()});
        }

        Err(anyhow!("path is not a directory"))
    }

    fn process_image(&self, data: Vec<u8>, content_type: String) -> Result<(), PersistenceError> {

        todo!()
    }
}

impl PersistenceService for LocalPersistenceService {
    async fn persist_file(&self, data: Vec<u8>, content_type: String, filename: String) -> Result<PersistenceSuccess, PersistenceError> {
        let hash = blake3::hash(&data);

        let img = image::load_from_memory(&data)
            .context("loading image failed")?;
        // Create the WebP encoder for the above image
        let encoder = webp::Encoder::from_image(&img).unwrap();
        // Encode the image at a specified quality 0-100
        let webp = encoder.encode(75f32);

        let mut processed_img = NamedTempFile::new().unwrap();
        let t = processed_img.write(webp.as_bytes()).unwrap();

        let process_img_name = format!("{}.webp", uuid::Uuid::new_v4());
        processed_img.persist(format!("./content/{}", process_img_name)).unwrap();

        Ok(PersistenceSuccess { filename: "test".to_string(), hash: hash.to_string(), content_type: "image/webp".to_string() })
    }

    async fn get_file(&self, hash: String, filename: String) -> Result<FilePayload, PersistenceError> {
        todo!()
    }
}