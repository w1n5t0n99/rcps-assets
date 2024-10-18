use std::future::Future;

use axum_typed_multipart::FieldData;
use tempfile::NamedTempFile;
use thiserror::Error;

use super::models::{Attachment, FilePayload};


#[derive(Error, Debug)]
pub enum FilesystemRepositoryError {
    #[error("uploaded file missing metadata")]
    MissingMetadata,
    #[error("invalid file type")]
    InvalidFileType,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

pub trait FilesystemRepository {
    fn persist_field_as_attachment(&self, field: FieldData<NamedTempFile>) -> impl Future<Output = Result<Attachment, FilesystemRepositoryError>> + Send; 
    fn get_file(&self, id: i32) -> impl Future<Output = Result<FilePayload, FilesystemRepositoryError>> + Send; 
    fn get_attachent_hash(&self, hash: String)-> impl Future<Output = Result<Attachment, FilesystemRepositoryError>> + Send;
}