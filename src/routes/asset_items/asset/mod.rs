mod edit;
pub use edit::{edit_asset, edit_asset_form};
use actix_web::http::header::{CacheControl, CacheDirective};
use actix_web_flash_messages::{IncomingFlashMessages, Level};
use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, web};
use sailfish::TemplateOnce;
use sqlx::PgPool;
use crate::domain::{Asset, AssetTemplate};
use crate::utils::e500;


#[tracing::instrument( 
    name = "View asset",
    skip(flash_messages, path, pool),
    fields(asset_id=tracing::field::Empty)
)]
pub async fn get_asset(flash_messages: IncomingFlashMessages, path: web::Path<uuid::Uuid>, pool: web::Data<PgPool>) -> Result<HttpResponse, actix_web::Error> {
    let id = path.into_inner();
    let error_messages: Vec<(Level, String)> = flash_messages.iter()
        .map(|m| {
            (m.level(), m.content().to_string())     
        })
        .collect();

    tracing::Span::current().record("id", &tracing::field::display(&id));

    let asset = retrieve_asset(&pool, id)
        .await
        .map_err(e500)?;

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
