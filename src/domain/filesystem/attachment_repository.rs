use std::future::Future;

use axum_typed_multipart::FieldData;
use tempfile::NamedTempFile;
use thiserror::Error;

use super::models::Attachment;


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
    fn get_attachent_from_hash(&self, hash: String)-> impl Future<Output = Result<Attachment, FilesystemRepositoryError>> + Send;
}