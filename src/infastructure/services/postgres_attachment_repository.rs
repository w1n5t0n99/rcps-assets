use std::io::Read;

use anyhow::Context;
use axum_typed_multipart::FieldData;
use sqlx::{postgres::{PgConnectOptions, PgPoolOptions, PgSslMode}, PgPool};
use tempfile::NamedTempFile;

use crate::{domain::filesystem::{attachment_repository::{AttachmentRepository, AttachmentRepositoryError}, models::{Attachment, NewAttachment}}, settings::DatabaseConfig};


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
    async fn get_attachent_from_hash(&self, hash: String)-> Result<Option<Attachment>, AttachmentRepositoryError> {
        let attachment = sqlx::query_as!(
            Attachment,
            r#"
            SELECT * FROM attachments
            WHERE $1 = hash
            "#,
            hash
        )
        .fetch_optional(&self.pool)
        .await
        .context("could not retrieve attachment from database")?;

        Ok(attachment)
    }

    async fn add_attachent(&self, new_attachment: NewAttachment)-> Result<Attachment, AttachmentRepositoryError> {
        let attachment = sqlx::query_as!(
            Attachment,
            r#"
            INSERT INTO attachments (hash, filename, content_type, url)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
            new_attachment.hash,
            new_attachment.filename,
            new_attachment.content_type,
            new_attachment.url,
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



