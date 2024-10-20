use std::io::Read;

use anyhow::Context;
use axum_typed_multipart::FieldData;
use sqlx::{postgres::{PgConnectOptions, PgPoolOptions, PgSslMode}, PgPool};
use tempfile::NamedTempFile;

use crate::{domain::filesystem::{attachment_repository::{FilesystemRepository, FilesystemRepositoryError}, models::Attachment}, settings::DatabaseConfig};


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

}



