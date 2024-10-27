use anyhow::{anyhow, Context};
use image::EncodableLayout;
use tempfile::NamedTempFile;
use tokio_util::io::ReaderStream;
use std::{fs, io::Write, path::{Path, PathBuf}};

use crate::{domain::filesystem::{image_utils::process_image, models::{ContentType, ExtensionType, FilePayload, Filename, NewDocumentAttachment, NewImageAttachment, MIME_LOOKUP}, persistence_service::{PersistenceError, PersistenceService}}, settings::LocalStorageConfig};


#[derive(Debug, Clone)]
pub struct LocalPersistenceService {
    pub images_path: PathBuf,
    pub docs_path: PathBuf,
    public_url: String,
}

impl LocalPersistenceService {
    pub fn new(config: &LocalStorageConfig) -> anyhow::Result<Self> {
        let images_path = Path::new(&config.local_directory_path).join("images");
        let docs_path = Path::new(&config.local_directory_path).join("documents");

        fs::create_dir_all(images_path.clone()).context("could not create image directory");
        fs::create_dir_all(docs_path.clone()).context("could not create documents directory");

        Ok(Self {
            images_path,
            docs_path,
            public_url: config.public_url.clone(),
        })
    }
}

impl PersistenceService for LocalPersistenceService {
    /*
    async fn persist_file(&self, payload: FilePayload, base_url: String) -> Result<NewAttachment, PersistenceError> {

        /*
        let ext = MIME_LOOKUP.get(payload.content_type.as_str()).ok_or(PersistenceError::ExtNotSupported)?;
        if ext.ext_type() == ExtensionType::Image {
            let (send, recv) = tokio::sync::oneshot::channel();

            rayon::spawn(move || {
                let res = process_image(payload.data, ext.clone());
                let _ = send.send(res);
            });

            let (processed_data, processed_ext)= recv.await??;

            let mut processed_img = NamedTempFile::new().unwrap();
            let _ = processed_img.write(&processed_data).unwrap();

            let processed_img_name = format!("{}.webp", uuid::Uuid::new_v4());
            processed_img.persist(self.local_directory.join(&processed_img_name)).unwrap();

            let url = format!("{}/{}/{}",base_url, &payload.hash, &processed_img_name);

            // we store the original hash so it can be used for deduplication
            return Ok(NewAttachment { filename: processed_img_name, hash: payload.hash, content_type: "image/webp".to_string(), url: url });

        } else {
            let mut tmp_file = NamedTempFile::new().unwrap();
            let _ = tmp_file.write(&payload.data).unwrap();

            // TODO: uses original filename for text and application files, might need to change 
            tmp_file.persist(self.local_directory.join(&payload.filename)).unwrap();

            let url = format!("{}/{}/{}",base_url, &payload.hash, &payload.filename);

            return Ok(NewAttachment { filename: payload.filename, hash: payload.hash, content_type: payload.content_type, url: url });
        }
        */

        todo!()

    }
    async fn get_file(&self, attachment: Attachment) -> Result<FilePayload, PersistenceError> {     
        /* 
        let data = tokio::fs::read(self.local_directory.join(&attachment.filename))
            .await?;

        Ok(FilePayload {
            data,
            filename: attachment.filename,
            hash: attachment.hash,
            content_type: attachment.content_type,
        })

        */

        todo!()
    }
    */

    async fn persist_image_file(&self, payload: FilePayload) -> Result<NewImageAttachment, PersistenceError> {
        let ext = MIME_LOOKUP.get(payload.content_type.as_str()).ok_or(PersistenceError::ExtNotSupported)?;
        if ext.ext_type() != ExtensionType::Image {
            return Err(PersistenceError::ExtNotSupported);
        }

        let filename = format!("{}.{}", payload.hash, ext);
        let filepath = self.images_path.join(&filename);

        let url = format!("{}/images/{}", self.public_url, payload.hash);

        payload.temp_file.persist(&filepath).context(format!("could not perist tempfile to {:?}", filepath))?;

        Ok(NewImageAttachment {
            filename: Filename::new(filename),
            url: url,
            url_thumb: "".to_string(), //TODO: add thumbnail support
            content_type: ContentType::new(payload.content_type),
            hash: payload.hash,
        })
    }

    async fn persist_document_file(&self, payload: FilePayload) -> Result<NewDocumentAttachment, PersistenceError> {
        let ext = MIME_LOOKUP.get(payload.content_type.as_str()).ok_or(PersistenceError::ExtNotSupported)?;
        if ext.ext_type() == ExtensionType::Image {
            return Err(PersistenceError::ExtNotSupported);
        }

        let filename = format!("{}.{}", payload.hash, ext);
        let filepath = self.docs_path.join(&filename);

        let url = format!("{}/documents/{}", self.public_url, payload.hash);

        payload.temp_file.persist(&filepath).context(format!("could not perist tempfile to {:?}", filepath))?;

        Ok(NewDocumentAttachment {
            filename: Filename::new(filename),
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