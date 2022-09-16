use actix_web::http::header::ContentType;
use actix_web::http::header::{CacheControl, CacheDirective};
use actix_web::{HttpResponse, web};
use anyhow::Context;
use sailfish::TemplateOnce;
use sqlx::PgPool;
use crate::domain::{Asset, EditAssetTemplate};
use crate::utils::{e500, RedirectError};


#[tracing::instrument( 
    name = "Edit asset form",
    skip(path, pool),
    fields(asset_id=tracing::field::Empty)
)]
pub async fn edit_asset_form(path: web::Path<i32>, pool: web::Data<PgPool>) -> Result<HttpResponse, actix_web::Error> {
    let id = path.into_inner();
    tracing::Span::current().record("id", &tracing::field::display(id));

    let asset = retrieve_asset(&pool, id)
        .await
        .map_err(e500)?;

    let body = EditAssetTemplate{asset: asset}.render_once().map_err(e500)?;

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