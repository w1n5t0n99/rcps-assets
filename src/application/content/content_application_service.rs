use std::io::Read;

use anyhow::Context;

use crate::{domain::filesystem::{attachment_repository::AttachmentRepository, models::{Attachment, FilePayload}, persistence_service::PersistenceService}, infastructure::services::{local_persistence_service::LocalPersistenceService, postgres_attachment_repository::PostgresAttachmentRepository}};

use super::schema::SingleUploadSchema;


#[derive(Debug, thiserror::Error)]
pub enum ContentError {
    #[error("the file metadata is corrupted or missing")]
    InvalidFileMetadata,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Clone)]
pub struct ContentApplicationService {
    attachment_repo: PostgresAttachmentRepository,
    persistence: LocalPersistenceService,
}

impl ContentApplicationService {
    pub fn new(attachment_repo: PostgresAttachmentRepository, persistence: LocalPersistenceService) -> Self {
        Self {
            attachment_repo,
            persistence,
        }
    }

    pub async fn upload_file_as_attachment(&self, upload: SingleUploadSchema) -> Result<Attachment, ContentError> {
        let payload = self.create_file_payload(&upload).await?;

        let attachment = self.attachment_repo.get_attachent_from_hash(payload.hash.clone())
            .await
            .context("could not retrieve attachment from database")?;

        if attachment.is_some() {
            return Ok(attachment.unwrap());
        }

        let new_attachment = self.persistence.persist_file(payload)
            .await
            .context("error persisting file")?;

        let attachment = self.attachment_repo.add_attachent(new_attachment)
            .await
            .context("could not add new attachment to database")?;

        Ok(attachment)
    }

    pub async fn retrieve_file_from_attachment(&self, attachment: Attachment) -> Result<FilePayload, ContentError> {

        todo!()
    }

    async fn create_file_payload(&self, upload: &SingleUploadSchema) -> Result<FilePayload, ContentError> {
        let filename = upload.field.metadata.file_name.clone().ok_or(ContentError::InvalidFileMetadata)?;
        let content_type = upload.field.metadata.content_type.clone().ok_or(ContentError::InvalidFileMetadata)?;

        let mut data = Vec::new();
        upload.field.contents.as_file().read_to_end(&mut data).unwrap();

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