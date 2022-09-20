use actix_web::http::header::{CacheControl, CacheDirective};
use actix_web_flash_messages::IncomingFlashMessages;
use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, web};
use sailfish::TemplateOnce;
use sqlx::PgPool;
use anyhow::Context;
use crate::domain::{Asset, AssetTemplate};
use crate::utils::get_success_messages;
use crate::errors::AssetsError;


#[tracing::instrument( 
    name = "View asset",
    skip(flash_messages, path, pool),
    fields(asset_id=tracing::field::Empty)
)]
pub async fn get_asset(
    flash_messages: IncomingFlashMessages,
    path: web::Path<i32>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AssetsError> {
    let messages = get_success_messages(flash_messages);

    let id = path.into_inner();
    tracing::Span::current().record("id", &tracing::field::display(&id));

    let asset = retrieve_asset(&pool, id)
        .await?;

    let body = AssetTemplate{messages, asset}
        .render_once()
        .context("Failed to parse template")?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .insert_header(CacheControl( vec![
            CacheDirective::NoCache,
            CacheDirective::NoStore,
            CacheDirective::MustRevalidate,
        ]))
        .body(body))
}

#[tracing::instrument(name = "Retrieve asset from database", skip(pool, id))]
async fn retrieve_asset(pool: &PgPool, id: i32) -> Result<Asset, sqlx::Error> {

    let r = sqlx::query_as!(
        Asset,
        r#"SELECT sid, asset_id, name, serial_num, model, brand, date_added FROM assets WHERE sid = $1"#,
        id,
    )
    .fetch_one(pool)
    .await?;

    Ok(r)
}
