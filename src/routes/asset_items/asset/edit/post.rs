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
    path: web::Path<i32>,
    form: web::Form<Asset>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let asset = { 
        let mut a = form.0;
        a.sid = path.into_inner();
        a
    };

    asset.validate()
        .context("Failed to convert form to asset.")
        .map_err(|e| {
            FlashMessage::error("Invalid user input.".to_string()).send();
            RedirectError::new(e, format!("/asset_items/{}", asset.sid))
        })?;

    let mut transaction = pool.begin()
        .await
        .context("Failed to acquire a Postgres connection from the pool")
        .map_err(e500)?;

    update_asset(&mut transaction, &asset)
        .await
        .context("Failed to update asset in database")
        .map_err(|e| {
            FlashMessage::error("Could not update asset".to_string()).send();
            RedirectError::new(e, format!("/asset_items/{}", asset.sid))
        })?;

    transaction
        .commit()
        .await
        .context("Failed to commit SQL transaction to store a new subscriber.")
        .map_err(e500)?;

    FlashMessage::success("Asset successfully added.".to_string()).send();
    // Asset was updated redirect to new ID
    Ok(see_other(format!("/asset_items/{}", asset.sid).as_str()))
}

#[tracing::instrument(name = "Updating asset details in database", skip(transaction, asset))]
async fn update_asset( transaction: &mut Transaction<'_, Postgres>, asset: &Asset ) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE assets SET asset_id = $1, name = $2, serial_num = $3, model = $4, brand = $5
        WHERE sid = $6
        "#,
        asset.asset_id,
        asset.name,
        asset.serial_num,
        asset.model,
        asset.brand,
        asset.sid,
    )
    .execute(transaction)
    .await?;
    
    Ok(())
}