use actix_web::http::StatusCode;
use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse, ResponseError};
use actix_web::error::InternalError;
use anyhow::Context;
use sqlx::{PgPool, Postgres, Transaction};
use validator::Validate;
use chrono::Utc;
use actix_web_flash_messages::FlashMessage;

use crate::utils::{error_chain_fmt, see_other, e500};
use crate::domain::Asset;


#[derive(thiserror::Error)]
pub enum AddAssetError {
    #[error("Failed to validate asset data")]
    ValidationError(#[source] anyhow::Error),
    #[error("Failed to insert asset data")]
    InsertError(#[source] anyhow::Error),
    #[error("Unexpected Error")]
    UnexpectedError(#[source] anyhow::Error),
}

impl std::fmt::Debug for AddAssetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for AddAssetError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match self {
            AddAssetError::ValidationError(e) => {
                FlashMessage::error("Invalid user input.".to_string()).send();
                see_other("/assets/add")
            },
            AddAssetError::InsertError(e) => {
                FlashMessage::error("Could not add asset".to_string()).send();
                see_other("/assets/add")
            },
            AddAssetError::UnexpectedError(e) => {
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                .insert_header(ContentType::html())
                .body(self.to_string())
            },
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
    form: web::Form<Asset>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AddAssetError> {

    let asset = form.0;
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
        .map_err(AddAssetError::InsertError)?;

    transaction
        .commit()
        .await
        .context("Failed to commit SQL transaction to store a new subscriber.")
        .map_err(AddAssetError::UnexpectedError)?;
    
    FlashMessage::success("Asset successfully added.".to_string()).send();
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
