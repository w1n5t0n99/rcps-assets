use actix_web::http::StatusCode;
use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse, ResponseError};
use anyhow::Context;
use sqlx::{PgPool, Postgres, Transaction};
use validator::Validate;
use chrono::Utc;
use actix_web_flash_messages::FlashMessage;
use actix_multipart::Multipart;


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

    todo!()
}