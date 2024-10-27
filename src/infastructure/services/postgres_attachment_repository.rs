use std::io::Read;

use anyhow::Context;
use axum_typed_multipart::FieldData;
use sqlx::{postgres::{PgConnectOptions, PgPoolOptions, PgSslMode}, PgPool};
use tempfile::NamedTempFile;

use crate::{domain::filesystem::{attachment_repository::{AttachmentRepository, AttachmentRepositoryError}, models::{ImageAttachment, NewImageAttachment, DocumentAttachment, NewDocumentAttachment, Filename, ContentType}}, settings::DatabaseConfig};


const UNIQUE_CONSTRAINT_VIOLATION_CODE: &str = "23505";

fn is_unique_constraint_violation(err: &sqlx::Error) -> bool {
    if let sqlx::Error::Database(db_err) = err {
        if let Some(code) = db_err.code() {
            if code == UNIQUE_CONSTRAINT_VIOLATION_CODE {
                return true;
            }
        }
    }

    false
}

#[derive(Debug, Clone)]
pub struct PostgresAttachmentRepository {
    pool: PgPool,
}

impl PostgresAttachmentRepository {
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
}

impl AttachmentRepository for PostgresAttachmentRepository {
    async fn get_image_attachent_from_hash(&self, hash: String)-> Result<Option<ImageAttachment>, AttachmentRepositoryError> {
        let attachment = sqlx::query_as!(
            ImageAttachment,
            r#"
            SELECT id, filename as "filename: Filename", hash, content_type as "content_type: ContentType", url, url_thumb, created_at
            FROM image_attachments
            WHERE $1 = hash
            "#,
            hash
        )
        .fetch_optional(&self.pool)
        .await
        .context("could not retrieve attachment from database")?;

        Ok(attachment)
    }

    async fn get_document_attachent_from_hash(&self, hash: String)-> Result<Option<DocumentAttachment>, AttachmentRepositoryError> {
        let attachment = sqlx::query_as!(
            DocumentAttachment,
            r#"
            SELECT id, filename as "filename: Filename", hash, content_type as "content_type: ContentType", url, description, created_at
            FROM document_attachments
            WHERE $1 = hash
            "#,
            hash
        )
        .fetch_optional(&self.pool)
        .await
        .context("could not retrieve attachment from database")?;

        Ok(attachment)
    }

    async fn add_image_attachent(&self, new_attachment: NewImageAttachment)-> Result<ImageAttachment, AttachmentRepositoryError> {
        let attachment = sqlx::query_as!(
            ImageAttachment,
            r#"
            INSERT INTO image_attachments (hash, filename, content_type, url ,url_thumb)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, filename as "filename: Filename", hash, content_type as "content_type: ContentType", url, url_thumb, created_at
            "#,
            new_attachment.hash,
            new_attachment.filename.to_string(),
            new_attachment.content_type.to_string(),
            new_attachment.url,
            new_attachment.url_thumb,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            if is_unique_constraint_violation(&e) == true { AttachmentRepositoryError::Duplicate }
            else { AttachmentRepositoryError::Unknown(e.into()) }
        })?;

        Ok(attachment)
    }

    async fn add_document_attachent(&self, new_attachment: NewDocumentAttachment, description: String)-> Result<DocumentAttachment, AttachmentRepositoryError> {
        let attachment = sqlx::query_as!(
            DocumentAttachment,
            r#"
            INSERT INTO document_attachments (hash, filename, content_type, url ,description)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, filename as "filename: Filename", hash, content_type as "content_type: ContentType", url, description, created_at
            "#,
            new_attachment.hash,
            new_attachment.filename.to_string(),
            new_attachment.content_type.to_string(),
            new_attachment.url,
            description,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            if is_unique_constraint_violation(&e) == true { AttachmentRepositoryError::Duplicate }
            else { AttachmentRepositoryError::Unknown(e.into()) }
        })?;

        Ok(attachment)
    }
}



