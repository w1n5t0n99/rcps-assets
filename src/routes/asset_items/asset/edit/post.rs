use actix_web::{web, HttpResponse,};
use anyhow::Context;
use sqlx::{PgPool, Postgres, Transaction};
use validator::Validate;
use actix_web_flash_messages::FlashMessage;

use crate::utils::{see_other, RedirectError, e500};
use crate::domain::Asset;


#[tracing::instrument(
    name = "Edit asset",
    skip(path, form, pool),
)]
pub async fn edit_asset(
    path: web::Path<String>,
    form: web::Form<Asset>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let current_asset_id = path.into_inner();
    let asset = form.0;

    asset.validate()
        .context("Failed to convert form to asset.")
        .map_err(|e| {
            FlashMessage::error("Invalid user input.".to_string()).send();
            RedirectError::E400(e, format!("/asset_items/{}", current_asset_id))
        })?;

    let mut transaction = pool.begin()
        .await
        .context("Failed to acquire a Postgres connection from the pool")
        .map_err(e500)?;

    update_asset(&mut transaction, &asset, &current_asset_id)
        .await
        .context("Failed to update asset in database")
        .map_err(|e| {
            FlashMessage::error("Could not update asset".to_string()).send();
            RedirectError::E500(e, format!("/asset_items/{}", current_asset_id))
        })?;

    transaction
        .commit()
        .await
        .context("Failed to commit SQL transaction to store a new subscriber.")
        .map_err(e500)?;

    FlashMessage::success("Asset successfully added.".to_string()).send();
    // Asset was updated redirect to new ID
    Ok(see_other(format!("/asset_items/{}", asset.asset_id).as_str()))
}

#[tracing::instrument(name = "Updating asset details in database", skip(transaction, asset, current_asset_id))]
async fn update_asset(
    transaction: &mut Transaction<'_, Postgres>,
    asset: &Asset,
    current_asset_id: &str,
) -> Result<(), sqlx::Error> {

    sqlx::query!(
        r#"
        UPDATE assets SET asset_id = $1, name = $2, serial_num = $3, model = $4, brand = $5
        WHERE asset_id = $6
        "#,
        asset.asset_id,
        asset.name,
        asset.serial_num,
        asset.model,
        asset.brand,
        current_asset_id,
    )
    .execute(transaction)
    .await?;
    
    Ok(())
}