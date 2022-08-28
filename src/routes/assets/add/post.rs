use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use anyhow::Context;
use sqlx::{PgPool, Postgres, Transaction};
use validator::Validate;
use chrono::Utc;

use crate::utils::{error_chain_fmt, see_other};
use crate::domain::Asset;

#[derive(serde::Deserialize)]
pub struct FormData {
    pub asset_id: String,
    pub name: String,
    pub serial_num: String,
    pub brand: String,
    pub model: String,
}

impl From<FormData> for Asset {
    fn from(a: FormData) -> Self {
        Asset {
            asset_id: a.asset_id,
            name: a.name,
            serial_num: a.serial_num,
            brand: a.brand,
            model: a.model,
        }
    }
}

#[derive(thiserror::Error)]
pub enum AddAssetError {
    #[error("Failed to validate asset data")]
    ValidationError(#[source] anyhow::Error),
    #[error("Unexpected Error")]
    UnexpectedError(#[source] anyhow::Error),
}

impl std::fmt::Debug for AddAssetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for AddAssetError {
    fn status_code(&self) -> StatusCode {
        match self {
            AddAssetError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AddAssetError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[tracing::instrument(
    name = "Add an asset",
    skip(form, pool),
    fields(
        asset_id = %form.asset_id,
        asset_name = %form.name,
        serial_num = %form.serial_num
    )
)]
pub async fn add_asset(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AddAssetError> {

    let asset = Asset::from(form.0);
    asset.validate()
        .context("Failed to convert form to asset.")
        .map_err(AddAssetError::ValidationError)?;

    let mut transaction = pool.begin()
        .await
        .context("Failed to acquire a Postgres connection from the pool")
        .map_err(AddAssetError::UnexpectedError)?;

    insert_asset(&mut transaction, &asset)
        .await
        .context("Failed to insert asset into database")
        .map_err(AddAssetError::UnexpectedError)?;

    transaction
        .commit()
        .await
        .context("Failed to commit SQL transaction to store a new subscriber.")
        .map_err(AddAssetError::UnexpectedError)?;
    
    Ok(see_other("/"))
}

#[tracing::instrument(name = "Saving new asset details into database", skip(transaction, asset))]
async fn insert_asset(
    transaction: &mut Transaction<'_, Postgres>,
    asset: &Asset,
) -> Result<(), sqlx::Error> {
    let id = uuid::Uuid::new_v4();

    sqlx::query!(
        r#"
        INSERT INTO assets (id, asset_id, name, serial_num, model, brand, date_added)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        id,
        asset.asset_id,
        asset.name,
        asset.serial_num,
        asset.model,
        asset.brand,
        Utc::now()
    )
    .execute(transaction)
    .await?;
    
    Ok(())
}