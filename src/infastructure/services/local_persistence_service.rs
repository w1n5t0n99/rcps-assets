use anyhow::Context;
use tempfile::NamedTempFile;
use std::{fs, path::{Path, PathBuf}};

use crate::domain::filesystem::{models::{ContentType, ExtensionType, FilePayload, Filename, NewDocumentAttachment, NewImageAttachment, MIME_LOOKUP}, persistence_service::{PersistenceError, PersistenceService}};


#[derive(Debug, Clone)]
pub struct LocalPersistenceService {
    pub images_path: PathBuf,
    pub docs_path: PathBuf,
    route_path: String,
}

impl LocalPersistenceService {
    pub fn new(route_path: &str, serve_path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let base = PathBuf::from(".");
        let images_path = base.join(&serve_path).join("images");
        let docs_path = base.join(&serve_path).join("documents");

        fs::create_dir_all(images_path.clone()).context("could not create image directory")?;
        fs::create_dir_all(docs_path.clone()).context("could not create documents directory")?;

        Ok(Self {
            images_path,
            docs_path,
            route_path: route_path.to_string(),
        })
    }
}

impl PersistenceService for LocalPersistenceService {
    async fn persist_image_file(&self, payload: FilePayload) -> Result<NewImageAttachment, PersistenceError> {
        let ext = MIME_LOOKUP.get(payload.content_type.as_str()).ok_or(PersistenceError::ExtNotSupported)?;
        if ext.ext_type() != ExtensionType::Image {
            return Err(PersistenceError::ExtNotSupported);
        }


        let persisted_filename = format!("{}.{}", payload.hash, ext);
        let filepath = self.images_path.join(&persisted_filename);
        let url = format!("{}/images/{}", self.route_path, &persisted_filename);

        payload.temp_file.persist(&filepath).context(format!("could not perist tempfile to {:?}", filepath))?;

        Ok(NewImageAttachment {
            filename: Filename::new(payload.filename),
            url: url,
            content_type: ContentType::new(payload.content_type),
            hash: payload.hash,
        })
    }

    async fn persist_document_file(&self, payload: FilePayload) -> Result<NewDocumentAttachment, PersistenceError> {
        let ext = MIME_LOOKUP.get(payload.content_type.as_str()).ok_or(PersistenceError::ExtNotSupported)?;
        if ext.ext_type() == ExtensionType::Image {
            return Err(PersistenceError::ExtNotSupported);
        }

        let persisted_filename = format!("{}.{}", payload.hash, ext);
        let filepath = self.docs_path.join(&persisted_filename);
        let url = format!("{}/documents/{}", self.route_path, &persisted_filename);

        payload.temp_file.persist(&filepath).context(format!("could not perist tempfile to {:?}", filepath))?;

        Ok(NewDocumentAttachment {
            filename: Filename::new(payload.filename),
            url: url,
            content_type: ContentType::new(payload.content_type),
            hash: payload.hash,
        })
    }

    async fn hash_file(&self, file: NamedTempFile)-> Result<(NamedTempFile, String), PersistenceError> {
        let (f, h) = tokio::task::spawn_blocking(move|| {
            let mut hasher = blake3::Hasher::new();
            hasher.update_mmap_rayon(file.path()).unwrap();

            (file, hasher.finalize().to_string())
        })
        .await?;

        Ok((f, h))
    }
}