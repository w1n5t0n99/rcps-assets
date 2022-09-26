use actix_web::{web, HttpResponse,};
use sqlx::{PgPool, Postgres, Transaction};
use actix_web_flash_messages::FlashMessage;

use crate::utils::see_other;
use crate::errors::AssetsError;


#[tracing::instrument(
    name = "Delete asset",
    skip(path, pool),
)]
pub async fn delete_asset(
    path: web::Path<i32>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AssetsError> {

    let mut transaction = pool.begin()
        .await?;

    delete_asset_from_db(&mut transaction, path.into_inner())
        .await
        .map_err(|e| {
            FlashMessage::error("Could not delete asset".to_string()).send();
            e
        })?;

    transaction
        .commit()
        .await?;


    FlashMessage::success("Asset successfully deleted.".to_string()).send();
    Ok(see_other(format!("/asset_items").as_str()))
}

#[tracing::instrument(name = "Deleting asset details in database", skip(transaction, id))]
async fn delete_asset_from_db( transaction: &mut Transaction<'_, Postgres>, id: i32 ) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM assets WHERE sid = $1
        "#,
        id,
    )
    .execute(transaction)
    .await?;
    
    Ok(())
}