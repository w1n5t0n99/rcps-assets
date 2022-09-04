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
pub async fn edit_asset_form(path: web::Path<String>, pool: web::Data<PgPool>) -> Result<HttpResponse, actix_web::Error> {
    let id = uuid::Uuid::parse_str(path.into_inner().as_str())
        .context("Failed to parse id")
        .map_err(|e| RedirectError::E500(e, "/asset_items".to_string()))?;

    tracing::Span::current().record("asset_id", &tracing::field::display(id));

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
async fn retrieve_asset(pool: &PgPool, id: uuid::Uuid) -> Result<Asset, sqlx::Error> {

    let r = sqlx::query!(
        r#"SELECT * FROM assets WHERE id = $1"#,
        id,
    )
    .fetch_one(pool)
    .await?;

    Ok(
        Asset {
            id: r.id,
            asset_id: r.asset_id,
            name: r.name,
            serial_num: r.serial_num,
            model: r.model.unwrap_or("".to_string()),
            brand: r.brand.unwrap_or("".to_string()),
        }
    )
}