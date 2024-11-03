use anyhow::Context;
use compact_str::{CompactString, ToCompactString};
use futures::TryFutureExt;
use sqlx::{postgres::{PgConnectOptions, PgPoolOptions, PgSslMode}, PgPool};
use uuid::Uuid;

use crate::{domain::crud::{crud_repository::{CrudRepository, CrudRepositoryError}, model::asset_types::{AssetType, AssetTypeFilter, NewAssetType, UpdateAssetType, UploadResult}}, settings::DatabaseConfig};


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

    async fn get_asset_types_search(&self, filter: AssetTypeFilter) -> Result<Vec<AssetType>, CrudRepositoryError> {

        tracing::info!("{:?}", filter);
        let asset_types = match (filter.search.as_deref(), filter.sort.as_deref(), filter.order.as_deref()) {
            (Some(search), Some(sort), Some("ASC")) => {
                sqlx::query_as!(
                    AssetType,
                    r#"
                    SELECT id, brand, model, description, cost, picture, created_at
                    FROM asset_types
                    WHERE full_search @@ websearch_to_tsquery($1)
                    ORDER BY
                    CASE 
                        WHEN $2 = 'brand' THEN brand
                        WHEN $2 = 'model' THEN model
                    END ASC
                    "#,
                    search,
                    sort,
                )
                .fetch_all(&self.pool)
                .await
            },
            (Some(search), Some(sort), Some("DESC")) => {
                tracing::info!("{:?}", sort);

                sqlx::query_as!(
                    AssetType,
                    r#"
                    SELECT id, brand, model, description, cost, picture, created_at
                    FROM asset_types
                    WHERE full_search @@ websearch_to_tsquery($1)
                    ORDER BY
                    CASE 
                        WHEN $2 = 'brand' THEN brand
                        WHEN $2 = 'model' THEN model
                    END DESC
                    "#,
                    search,
                    sort,
                )
                .fetch_all(&self.pool)
                .await
            },
            (Some(search), None, None) => {
                sqlx::query_as!(
                    AssetType,
                    r#"
                    SELECT id, brand, model, description, cost, picture, created_at
                    FROM asset_types
                    WHERE full_search @@ websearch_to_tsquery($1)
                    "#,
                    search,
                )
                .fetch_all(&self.pool)
                .await
            },
            (None, Some(sort), Some("ASC")) => {
                sqlx::query_as!(
                    AssetType,
                    r#"
                    SELECT id, brand, model, description, cost, picture, created_at
                    FROM asset_types
                    ORDER BY
                    CASE 
                        WHEN $1 = 'brand' THEN brand
                        WHEN $1 = 'model' THEN model
                    END ASC
                    "#,
                    sort,
                )
                .fetch_all(&self.pool)
                .await
            },
            (None, Some(sort), Some("DESC")) => {
                sqlx::query_as!(
                    AssetType,
                    r#"
                    SELECT id, brand, model, description, cost, picture, created_at
                    FROM asset_types
                    ORDER BY
                    CASE 
                        WHEN $1 = 'brand' THEN brand
                        WHEN $1 = 'model' THEN model
                    END DESC
                    "#,
                    sort,
                )
                .fetch_all(&self.pool)
                .await
            },          
            _ => {
                sqlx::query_as!(
                    AssetType,
                    r#"
                    SELECT id, brand, model, description, cost, picture, created_at
                    FROM asset_types
                    "#,
                )
                .fetch_all(&self.pool)
                .await
            },
        }.context("could not retrieve asset types from database")?;
        

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
        let asset_type = match update_asset_type.picture {
            Some(picture) => {
                sqlx::query_as!(
                    AssetType,
                    r#"
                    UPDATE asset_types
                        SET brand = $1, model = $2, description = $3, cost = $4, picture = $5
                        WHERE id = $6
                        RETURNING id, brand, model, description, cost, picture, created_at
                    "#,
                    update_asset_type.brand.clone(),
                    update_asset_type.model.clone(),
                    update_asset_type.description.clone(),
                    update_asset_type.cost.clone(),
                    picture.clone(),
                    id,
                )
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| {
                    if is_unique_constraint_violation(&e) == true { CrudRepositoryError::Duplicate }
                    else { CrudRepositoryError::Unknown(e.into()) }
                })?
            },
            None => {
                sqlx::query_as!(
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
                })?
            },
        };

        Ok(asset_type)
    }

    async fn update_asset_type_picture(&self, id: i32, picture: String) -> Result<Option<AssetType>, CrudRepositoryError> {
        let asset_type = sqlx::query_as!(
            AssetType,
            r#"
            UPDATE asset_types
                SET picture = $1
                WHERE id = $2
                RETURNING id, brand, model, description, cost, picture, created_at
            "#,
            picture,
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

    async fn bulk_add_or_update_asset_type(&self, add_asset_types: &[NewAssetType]) -> Result<UploadResult, CrudRepositoryError> {
        let brands: Vec<String> = add_asset_types.iter().map(|a| a.brand.clone()).collect();
        let models: Vec<String> = add_asset_types.iter().map(|a| a.model.clone()).collect();
        let descriptions: Vec<Option<String>> = add_asset_types.iter().map(|a| a.description.as_ref().map(|d| d.clone())).collect();
        let costs: Vec<Option<String>> = add_asset_types.iter().map(|a| a.cost.as_ref().map(|c| c.clone())).collect();
        let pictures: Vec<Option<String>> = add_asset_types.iter().map(|a| a.picture.as_ref().map(|p| p.clone())).collect();

        let rows = sqlx::query_as!(
            AssetType,
            r#"
            INSERT INTO asset_types (brand, model, description, cost, picture)
            SELECT DISTINCT ON(brand, model) brand, model, description, cost, picture FROM(
                SELECT * FROM UNNEST (
                $1::TEXT[],
                $2::TEXT[],
                $3::TEXT[],
                $4::TEXT[],
                $5::TEXT[]
            ) AS t(brand, model, description, cost, picture)
            WHERE t.brand IS NOT NULL AND t.model IS NOT NULL
            ) AS bulk_query
            ON CONFLICT ON CONSTRAINT asset_types_brand_model_key DO
            UPDATE SET 
                description = excluded.description,
                cost = excluded.cost,
                picture = excluded.picture
            RETURNING id, brand, model, description, cost, picture, created_at
            "#,
            &brands,
            &models,
            &descriptions as _,
            &costs as _,
            &pictures as _,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("{}", e);
            CrudRepositoryError::Unknown(e.into())
        })?;

        Ok(UploadResult { total: brands.len(), processed: rows.len() })
    }

    async fn bulk_add_asset_type(&self, add_asset_types: &[NewAssetType]) -> Result<UploadResult, CrudRepositoryError> {
        let brands: Vec<String> = add_asset_types.iter().map(|a| a.brand.clone()).collect();
        let models: Vec<String> = add_asset_types.iter().map(|a| a.model.clone()).collect();
        let descriptions: Vec<Option<String>> = add_asset_types.iter().map(|a| a.description.as_ref().map(|d| d.clone())).collect();
        let costs: Vec<Option<String>> = add_asset_types.iter().map(|a| a.cost.as_ref().map(|c| c.clone())).collect();
        let pictures: Vec<Option<String>> = add_asset_types.iter().map(|a| a.picture.as_ref().map(|p| p.clone())).collect();

        let rows = sqlx::query_as!(
            AssetType,
            r#"
            INSERT INTO asset_types (brand, model, description, cost, picture)
            SELECT DISTINCT ON(brand, model) brand, model, description, cost, picture FROM(
                SELECT * FROM UNNEST (
                $1::TEXT[],
                $2::TEXT[],
                $3::TEXT[],
                $4::TEXT[],
                $5::TEXT[]
            ) AS t(brand, model, description, cost, picture)
            WHERE t.brand IS NOT NULL AND t.model IS NOT NULL
            ) AS bulk_query
            ON CONFLICT ON CONSTRAINT asset_types_brand_model_key DO NOTHING
            RETURNING id, brand, model, description, cost, picture, created_at
            "#,
            &brands,
            &models,
            &descriptions as _,
            &costs as _,
            &pictures as _,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("{}", e);
            CrudRepositoryError::Unknown(e.into())
        })?;

        Ok(UploadResult { total: brands.len(), processed: rows.len() })
    }
}


