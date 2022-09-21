use actix_web::{web, HttpResponse};
use anyhow::Context;
use sqlx::{PgPool, Postgres, Transaction};
use validator::Validate;
use actix_web_flash_messages::FlashMessage;

use crate::utils::see_other;
use crate::domain::Asset;
use crate::errors::AssetsError;


#[tracing::instrument(
    name = "Add a new asset",
    skip(form, pool),
    fields(
        asset_id = %form.asset_id,
        asset_name = %form.name,
        serial_num = %form.serial_num
    )
)]
pub async fn new_asset(
    form: web::Form<Asset>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AssetsError> {

    let asset = form.0;
    asset.validate()?;

    let mut transaction = pool.begin()
        .await
        .context("Failed to acquire a Postgres connection from the pool")?;

    insert_asset(&mut transaction, &asset)
        .await?;

    transaction
        .commit()
        .await?;

    FlashMessage::success("Asset successfully added.".to_string()).send();
    Ok(see_other("/asset_items"))
}

#[tracing::instrument(name = "Saving new asset details into database", skip(transaction, asset))]
async fn insert_asset(
    transaction: &mut Transaction<'_, Postgres>,
    asset: &Asset,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO assets (asset_id, name, serial_num, model, brand, date_added)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        asset.asset_id,
        asset.name,
        asset.serial_num,
        asset.model,
        asset.brand,
        asset.date_added,
    )
    .execute(transaction)
    .await?;
    
    Ok(())
}
