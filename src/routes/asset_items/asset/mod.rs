mod edit;
pub use edit::{edit_asset, edit_asset_form};

use actix_web::http::header::{CacheControl, CacheDirective};
use actix_web_flash_messages::{IncomingFlashMessages, Level};
use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, web};
use sailfish::TemplateOnce;
use sqlx::PgPool;
use anyhow::Context;
use crate::domain::{Asset, AssetTemplate};
use crate::utils::{RedirectError, e500};


#[tracing::instrument( 
    name = "View asset",
    skip(flash_messages, path, pool),
    fields(asset_id=tracing::field::Empty)
)]
pub async fn get_asset(flash_messages: IncomingFlashMessages, path: web::Path<i32>, pool: web::Data<PgPool>) -> Result<HttpResponse, actix_web::Error> {
    let error_messages: Vec<(Level, String)> = flash_messages.iter()
        .map(|m| {
            (m.level(), m.content().to_string())     
        })
        .collect();

    let id = path.into_inner();
    tracing::Span::current().record("id", &tracing::field::display(&id));

    let asset = retrieve_asset(&pool, id)
        .await
        .context("Failed to retrieve asset")
        .map_err(|e| RedirectError::new(e, "/asset_items".to_string()))?;

    let body = AssetTemplate{messages: error_messages, asset: asset}.render_once().map_err(e500)?;

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
