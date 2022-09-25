use actix_web::{web, HttpResponse,};
use anyhow::Context;
use sqlx::{PgPool, Postgres, Transaction};
use validator::Validate;
use actix_web_flash_messages::FlashMessage;

use crate::utils::{see_other, RedirectError, e500};
use crate::domain::Asset;
use crate::errors::AssetsError;


#[tracing::instrument(
    name = "Edit asset",
    skip(path, form, pool),
)]
pub async fn edit_asset(
    path: web::Path<i32>,
    form: web::Form<Asset>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AssetsError> {
    let asset = { 
        let mut a = form.0;
        a.sid = path.into_inner();
        a
    };

    asset.validate()
        .map_err(|e| {
            FlashMessage::error("Invalid data for asset.".to_string()).send();
            e
        })?;

    let mut transaction = pool.begin()
        .await?;

    update_asset(&mut transaction, &asset)
        .await
        .map_err(|e| {
            FlashMessage::error("Could not update asset".to_string()).send();
            e
        })?;

    transaction
        .commit()
        .await?;

    FlashMessage::success("Asset successfully changed.".to_string()).send();
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