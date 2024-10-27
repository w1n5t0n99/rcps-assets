use std::{io::Read, path::Path};

use anyhow::Context;
use axum_typed_multipart::FieldData;
use tempfile::NamedTempFile;

use crate::{domain::filesystem::{attachment_repository::AttachmentRepository, models::{ContentType, DocumentAttachment, FilePayload, Filename, ImageAttachment, MIME_LOOKUP}, persistence_service::PersistenceService}, infastructure::services::{local_persistence_service::LocalPersistenceService, postgres_attachment_repository::PostgresAttachmentRepository}};


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
}

impl ContentApplicationService {
    pub fn new(attachment_repo: PostgresAttachmentRepository, persistence: LocalPersistenceService) -> Self {
        Self {
            attachment_repo,
            persistence,
        }
    }

    pub async fn retrieve_image_file(&self, hash: String) -> Result<(tokio::fs::File, Filename, ContentType), ContentError> {
        let attachment = self.attachment_repo.get_image_attachent_from_hash(hash)
            .await
            .context("could not retrieve attachment from database")?
            .ok_or(ContentError::Missing)?;

        let filepath = self.persistence.images_path.join(AsRef::<str>::as_ref(&attachment.filename));
        let file = tokio::fs::File::open(&filepath).await.map_err(|_| ContentError::Missing)?;

        Ok((file, attachment.filename, attachment.content_type))
    }

    pub async fn retrieve_document_file(&self, hash: String) -> Result<(tokio::fs::File, Filename, ContentType), ContentError> {
        let attachment = self.attachment_repo.get_document_attachent_from_hash(hash)
            .await
            .context("could not retrieve attachment from database")?
            .ok_or(ContentError::Missing)?;

        let filepath = self.persistence.docs_path.join(AsRef::<str>::as_ref(&attachment.filename));
        let file = tokio::fs::File::open(&filepath).await.map_err(|_| ContentError::Missing)?;

        Ok((file, attachment.filename, attachment.content_type))
    }

    pub async fn upload_image_file_as_attachment(&self, uploaded_file: FieldData<NamedTempFile>) -> Result<ImageAttachment, ContentError> {
        let payload = self.create_file_payload(uploaded_file).await?;

        let attachment = self.attachment_repo.get_image_attachent_from_hash(payload.hash.clone())
            .await
            .context("could not retrieve attachment from database")?;

        if attachment.is_some() {
            tracing::info!("duplicate attachment found");
            return Ok(attachment.unwrap());
        }

        let new_attachment = self.persistence.persist_image_file(payload)
            .await
            .context("error persisting file")?;

        let attachment = self.attachment_repo.add_image_attachent(new_attachment)
            .await
            .context("could not add new attachment to database")?;

        Ok(attachment)
    }

    pub async fn upload_document_file_as_attachment(&self, uploaded_file: FieldData<NamedTempFile>, description: String) -> Result<DocumentAttachment, ContentError> {
        let payload = self.create_file_payload(uploaded_file).await?;

        let attachment = self.attachment_repo.get_document_attachent_from_hash(payload.hash.clone())
            .await
            .context("could not retrieve attachment from database")?;

        if attachment.is_some() {
            tracing::info!("duplicate attachment found");
            return Ok(attachment.unwrap());
        }

        let new_attachment = self.persistence.persist_document_file(payload)
            .await
            .context("error persisting file")?;

        let attachment = self.attachment_repo.add_document_attachent(new_attachment, description)
            .await
            .context("could not add new attachment to database")?;

        Ok(attachment)
    }

    async fn create_file_payload(&self, uploaded_file: FieldData<NamedTempFile>) -> Result<FilePayload, ContentError> {
        let filename = uploaded_file.metadata.file_name.clone().ok_or(ContentError::InvalidFileMetadata)?;
        let content_type = uploaded_file.metadata.content_type.clone().ok_or(ContentError::InvalidFileMetadata)?;

        let (temp_file, hash) = self.persistence.hash_file(uploaded_file.contents)
            .await
            .context("error hashing file")?;

        Ok(FilePayload {
            filename,
            content_type,
            hash,
            temp_file,
        })
    }
}