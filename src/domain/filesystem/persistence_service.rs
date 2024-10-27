use std::future::Future;

use tempfile::NamedTempFile;
use thiserror::Error;
use tokio::{sync::oneshot::error::RecvError, task::JoinError};

use super::models::{ContentType, FilePayload, Filename, NewDocumentAttachment, NewImageAttachment};


#[derive(Error, Debug)]
pub enum PersistenceError {
    #[error("file type not supported")]
    ExtNotSupported,
    #[error(transparent)]
    Task(#[from] JoinError),
    #[error(transparent)]
    Worker(#[from] RecvError),
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

pub trait PersistenceService {
    fn hash_file(&self, file: NamedTempFile)-> impl Future<Output = Result<(NamedTempFile, String), PersistenceError>> + Send;
    fn persist_image_file(&self, payload: FilePayload) -> impl Future<Output = Result<NewImageAttachment, PersistenceError>> + Send;
    fn persist_document_file(&self, payload: FilePayload) -> impl Future<Output = Result<NewDocumentAttachment, PersistenceError>> + Send;
}