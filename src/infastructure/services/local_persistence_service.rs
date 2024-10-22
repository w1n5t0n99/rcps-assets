use anyhow::{anyhow, Context};
use image::EncodableLayout;
use tempfile::NamedTempFile;
use tokio_util::io::ReaderStream;
use std::{io::Write, path::{Path, PathBuf}};

use crate::domain::filesystem::{image_utils::process_image, models::{Attachment, ExtensionType, FilePayload, NewAttachment, MIME_LOOKUP}, persistence_service::{PersistenceError, PersistenceService}};


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
}

impl PersistenceService for LocalPersistenceService {
    async fn persist_file(&self, payload: FilePayload) -> Result<NewAttachment, PersistenceError> {

        let ext = MIME_LOOKUP.get(payload.content_type.as_str()).ok_or(PersistenceError::ExtNotSupported)?;
        if ext.ext_type() == ExtensionType::Image {
            let process_image_task = tokio::task::spawn_blocking(move || {
                process_image(payload.data, ext.clone())
            });

            let processed_results = process_image_task
                .await??;

            let mut processed_img = NamedTempFile::new().unwrap();
            let _ = processed_img.write(&processed_results.0).unwrap();

            let process_img_name = format!("{}.webp", uuid::Uuid::new_v4());
            processed_img.persist(self.content_directory.join(&process_img_name)).unwrap();

            // we store the original hash so it can be used for deduplication
            return Ok(NewAttachment { filename: process_img_name, hash: payload.hash, content_type: "image/webp".to_string() });

        } else {
            let mut tmp_file = NamedTempFile::new().unwrap();
            let _ = tmp_file.write(&payload.data).unwrap();

            // TODO: use original filename for text and application files, might eant to change 
            tmp_file.persist(self.content_directory.join(&payload.filename)).unwrap();

            return Ok(NewAttachment { filename: payload.filename, hash: payload.hash, content_type: payload.content_type });
        }

    }

    async fn get_file(&self, attachment: Attachment) -> Result<FilePayload, PersistenceError> {       
        let data = tokio::fs::read(self.content_directory.join(&attachment.filename))
            .await?;

        Ok(FilePayload {
            data,
            filename: attachment.filename,
            hash: attachment.hash,
            content_type: attachment.content_type,
        })
    }

    async fn hash_file(&self, data: Vec<u8>)-> Result<String, PersistenceError> {
        let hash = tokio::task::spawn_blocking(move|| {
                blake3::hash(&data)
        })
        .await?
        .to_string();

        Ok(hash)
    }
}