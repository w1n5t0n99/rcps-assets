use std::future::Future;

use axum_typed_multipart::FieldData;
use tempfile::NamedTempFile;
use thiserror::Error;

use super::models::{Attachment, NewAttachment};


#[derive(Error, Debug)]
pub enum AttachmentRepositoryError {
    #[error("attachment already exists")]
    Duplicate,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

pub trait AttachmentRepository {
    fn get_attachent_from_hash(&self, hash: String)-> impl Future<Output = Result<Option<Attachment>, AttachmentRepositoryError>> + Send;
    fn add_attachent(&self, new_attachment: NewAttachment)-> impl Future<Output = Result<Attachment, AttachmentRepositoryError>> + Send;
}