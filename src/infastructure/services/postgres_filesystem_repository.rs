use std::io::Read;

use anyhow::Context;
use axum_typed_multipart::FieldData;
use sqlx::{postgres::{PgConnectOptions, PgPoolOptions, PgSslMode}, PgPool};
use tempfile::NamedTempFile;

use crate::{domain::filesystem::{filesystem_repository::{FilesystemRepository, FilesystemRepositoryError}, models::{Attachment, FilePayload, MIME_LOOKUP}}, settings::DatabaseConfig};


#[derive(Debug, Clone)]
pub struct PostgresFilesystemRepository {
    pool: PgPool,
}

impl PostgresFilesystemRepository {
    pub fn new(config: &DatabaseConfig) -> anyhow::Result<Self> {
        let ssl_mode = if config.require_ssl == true { PgSslMode::Require } else { PgSslMode::Prefer };
        let pg_connect_options = PgConnectOptions::new()
            .host(&config.host)
            .username(&config.username)
            .password(&config.password)
            .port(config.port)
            .ssl_mode(ssl_mode)
            .database(&config.database_name);

        let pg_pool = PgPoolOptions::new().connect_lazy_with(pg_connect_options);

        Ok(Self { pool: pg_pool })
    }

    /// Accepts a multipart field, [stores it on the disk], and returns indentifying information about it.
    fn generate_payload(&self, field: &FieldData<NamedTempFile>) -> Result<FilePayload, FilesystemRepositoryError> {
        let mut hasher = blake3::Hasher::new();
        let mut data = Vec::new();

        field.contents.as_file().read_to_end(&mut data).context("tempfile io error")?;
        hasher.update(&mut data);
        let hash = hasher.finalize().to_string();

        let content_type = field.metadata.content_type.clone().ok_or(FilesystemRepositoryError::MissingMetadata)?;
        let ext = MIME_LOOKUP.get(content_type.as_str()).ok_or(FilesystemRepositoryError::InvalidFileType)?;
        let filename = format!("{}.{}", hash, ext);

        Ok(
            FilePayload {
                data,
                hash,
                filename,
                content_type,
            }
        )
    }
}

impl FilesystemRepository for PostgresFilesystemRepository {
    async fn persist_field_as_attachment(&self, field: FieldData<NamedTempFile>) -> Result<Attachment, FilesystemRepositoryError> {
        let payload = self.generate_payload(&field)?;
        //TODO: deduplicate - return if attachment found 

        todo!()
    }

    async fn get_file(&self, id: i32) -> Result<FilePayload, FilesystemRepositoryError> {
        todo!()
    }

    async fn get_attachent_hash(&self, hash: String)-> Result<Attachment, FilesystemRepositoryError> {
        todo!()
    }
}

