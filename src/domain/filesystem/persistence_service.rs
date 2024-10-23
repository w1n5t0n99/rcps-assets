use std::future::Future;

use thiserror::Error;
use tokio::{sync::oneshot::error::RecvError, task::JoinError};

use super::models::{Attachment, FilePayload, NewAttachment};


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
    ProcessingFailed(#[from] anyhow::Error),
}

pub trait PersistenceService {
    fn persist_file(&self, payload: FilePayload, base_url: String) -> impl Future<Output = Result<NewAttachment, PersistenceError>> + Send; 
    fn get_file(&self, attachment: Attachment) -> impl Future<Output = Result<FilePayload, PersistenceError>> + Send; 
    fn hash_file(&self, data: Vec<u8>)-> impl Future<Output = Result<String, PersistenceError>> + Send;
}