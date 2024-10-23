use std::io::Read;

use anyhow::Context;
use axum_typed_multipart::FieldData;
use tempfile::NamedTempFile;

use crate::{domain::filesystem::{attachment_repository::AttachmentRepository, models::{Attachment, FilePayload}, persistence_service::PersistenceService}, infastructure::services::{local_persistence_service::LocalPersistenceService, postgres_attachment_repository::PostgresAttachmentRepository}};


#[derive(Debug, thiserror::Error)]
pub enum ContentError {
    #[error("the file metadata is corrupted or missing")]
    InvalidFileMetadata,
    #[error("the file attachment could not be found")]
    Missing,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Clone)]
pub struct ContentApplicationService {
    attachment_repo: PostgresAttachmentRepository,
    persistence: LocalPersistenceService,
    content_url: String,
}

impl ContentApplicationService {
    pub fn new(attachment_repo: PostgresAttachmentRepository, persistence: LocalPersistenceService, content_url: String) -> Self {
        Self {
            attachment_repo,
            persistence,
            content_url
        }
    }

    pub async fn upload_file_as_attachment(&self, uploaded_file: FieldData<NamedTempFile>) -> Result<Attachment, ContentError> {
        let payload = self.create_file_payload(&uploaded_file).await?;

        let attachment = self.attachment_repo.get_attachent_from_hash(payload.hash.clone())
            .await
            .context("could not retrieve attachment from database")?;

        if attachment.is_some() {
            return Ok(attachment.unwrap());
        }

        let new_attachment = self.persistence.persist_file(payload, self.content_url.clone())
            .await
            .context("error persisting file")?;

        let attachment = self.attachment_repo.add_attachent(new_attachment)
            .await
            .context("could not add new attachment to database")?;

        Ok(attachment)
    }

    pub async fn retrieve_file(&self, hash: String) -> Result<FilePayload, ContentError> {
        let attachment = self.attachment_repo.get_attachent_from_hash(hash)
            .await
            .context("could not retrieve attachment from database")?
            .ok_or(ContentError::Missing)?;

        let payload = self.persistence.get_file(attachment)
            .await
            .context("could not retrieve file payload")?;

        Ok(payload)
    }

    async fn create_file_payload(&self, uploaded_file: &FieldData<NamedTempFile>) -> Result<FilePayload, ContentError> {
        let filename = uploaded_file.metadata.file_name.clone().ok_or(ContentError::InvalidFileMetadata)?;
        let content_type = uploaded_file.metadata.content_type.clone().ok_or(ContentError::InvalidFileMetadata)?;

        let mut data = Vec::new();
        uploaded_file.contents.as_file().read_to_end(&mut data).unwrap();

        let hash = self.persistence.hash_file(data.clone())
            .await
            .context("error hashing file")?;

        Ok(FilePayload {
            filename,
            content_type,
            hash,
            data
        })
    }
}