use anyhow::Context;
use futures::TryFutureExt;
use sqlx::{postgres::{PgConnectOptions, PgPoolOptions, PgSslMode}, PgPool};
use uuid::Uuid;

use crate::{domain::crud::{crud_repository::{CrudRepository, CrudRepositoryError}, model::asset_types::{AssetType, NewAssetType, UpdateAssetType}}, settings::DatabaseConfig};


#[derive(Debug, Clone)]
pub struct PostgresCrudRepository {
    pool: PgPool,
}

impl PostgresCrudRepository {
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


const UNIQUE_CONSTRAINT_VIOLATION_CODE: &str = "23505";
const FOREIGN_KEY_VIOLATION: &str = "23503";

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

// an insert or update operation has attempted to create a foreign key value in a child table
// that does not have a matching primary key value in the parent table
fn is_foreign_key_violation(err: &sqlx::Error) -> bool {
    if let sqlx::Error::Database(db_err) = err {
        if let Some(code) = db_err.code() {
            if code == FOREIGN_KEY_VIOLATION {
                return true;
            }
        }
    }

    false
}

impl CrudRepository for PostgresCrudRepository {
    async fn add_asset_type(&self, new_asset_type: NewAssetType) -> Result<AssetType, CrudRepositoryError> {
        let asset_type = sqlx::query_as!(
            AssetType,
            r#"
            INSERT INTO asset_types (brand, model, description, cost, picture)
            VALUES($1, $2, $3, $4, $5)
            RETURNING id, brand, model, description, cost, picture, created_at
            "#,
            new_asset_type.brand,
            new_asset_type.model,
            new_asset_type.description,
            new_asset_type.cost,
            new_asset_type.picture,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            if is_unique_constraint_violation(&e) == true { CrudRepositoryError::Duplicate }
            else { CrudRepositoryError::Unknown(e.into()) }
        })?;

        Ok(asset_type)
    }

    async fn get_asset_type_by_id(&self, id: i32) -> Result<Option<AssetType>, CrudRepositoryError> {
        let asset_type = sqlx::query_as!(
            AssetType,
            r#"
            SELECT id, brand, model, description, cost, picture, created_at
            FROM asset_types
            WHERE asset_types.id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .context("could not retrieve asset type from database")?;

        Ok(asset_type)
    }

    async fn get_asset_type(&self, brand: String, model: String) -> Result<Option<AssetType>, CrudRepositoryError> {
        let asset_type = sqlx::query_as!(
            AssetType,
            r#"
            SELECT id, brand, model, description, cost, picture, created_at
            FROM asset_types
            WHERE asset_types.brand = $1 AND asset_types.model = $2
            "#,
            brand,
            model,
        )
        .fetch_optional(&self.pool)
        .await
        .context("could not retrieve asset type from database")?;

        Ok(asset_type)
    }

    // TODO: add cursor pagination
    async fn get_asset_types(&self) -> Result<Vec<AssetType>, CrudRepositoryError> {
        let asset_types = sqlx::query_as!(
            AssetType,
            r#"
            SELECT id, brand, model, description, cost, picture, created_at
            FROM asset_types
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .context("could not retrieve asset types from database")?;

        Ok(asset_types)
        
    }

    async fn delete_asset_type(&self, id: i32) -> Result<Option<i32>, CrudRepositoryError> {
        let returned_id = sqlx::query!(
            r#"
            DELETE FROM asset_types WHERE id = $1
            RETURNING id
            "#,
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .context("could not delete asset from database")?;

        Ok(returned_id.map(|r| r.id))
    }

    async fn update_asset_type(&self, id: i32, update_asset_type: UpdateAssetType) -> Result<Option<AssetType>, CrudRepositoryError> {
        let asset_type = sqlx::query_as!(
            AssetType,
            r#"
            UPDATE asset_types
                SET brand = $1, model = $2, description = $3, cost = $4
                WHERE id = $5
                RETURNING id, brand, model, description, cost, picture, created_at
            "#,
            update_asset_type.brand.clone(),
            update_asset_type.model.clone(),
            update_asset_type.description.clone(),
            update_asset_type.cost.clone(),
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            if is_unique_constraint_violation(&e) == true { CrudRepositoryError::Duplicate }
            else { CrudRepositoryError::Unknown(e.into()) }
        })?;

        Ok(asset_type)
    }
}
