use actix_web::http::StatusCode;
use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse, ResponseError};
use actix_web_lab::__reexports::futures_util::TryStreamExt;
use anyhow::Context;
use sqlx::{PgPool, Postgres, Transaction};
use validator::Validate;
use chrono::Utc;
use actix_web_flash_messages::FlashMessage;
use actix_multipart::{Multipart, Field};

use crate::telemetry::spawn_blocking_with_tracing;
use crate::utils::{error_chain_fmt, see_other};
use crate::domain::Asset;


#[tracing::instrument(
    name = "Upload Assets from file",
    skip(payload, pool),
)]
pub async fn upload_assets(
    mut payload: Multipart,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition();

        //let filename = content_type.get_filename().unwrap();
        let filepath = format!("/temp/{}.csv", uuid::Uuid::new_v4());
        //FlashMessage::success(filepath).send();

        // File::create is blocking operation
        
    }

    Ok(see_other("/asset_items/uploads/new"))
}