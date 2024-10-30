use std::future::Future;

use axum_typed_multipart::FieldData;
use tempfile::NamedTempFile;
use thiserror::Error;

use super::models::{ImageAttachment, DocumentAttachment, NewImageAttachment, NewDocumentAttachment};


#[derive(Error, Debug)]
pub enum AttachmentRepositoryError {
    #[error("attachment already exists")]
    Duplicate,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

pub trait AttachmentRepository {
    fn get_image_attachent_from_hash(&self, hash: String)-> impl Future<Output = Result<Option<ImageAttachment>, AttachmentRepositoryError>> + Send;
    fn get_document_attachent_from_hash(&self, hash: String)-> impl Future<Output = Result<Option<DocumentAttachment>, AttachmentRepositoryError>> + Send;
    fn add_image_attachent(&self, new_attachment: NewImageAttachment)-> impl Future<Output = Result<ImageAttachment, AttachmentRepositoryError>> + Send;
    fn add_document_attachent(&self, new_attachment: NewDocumentAttachment)-> impl Future<Output = Result<DocumentAttachment, AttachmentRepositoryError>> + Send;
}