use actix_web::http::header::{ContentType, CacheControl, CacheDirective};
use actix_web::{HttpResponse, web, HttpRequest};
use sailfish::TemplateOnce;
use sqlx::PgPool;
use crate::utils::e500;
use crate::domain::{PartialAsset, AssetsTemplate};


#[derive(serde::Deserialize)]
pub struct QueryParams {
    name: Option<String>,
}

#[tracing::instrument( 
    name = "View asset items",
    skip(req, pool, query),
    fields(query=req.query_string())
)]
pub async fn asset_items_form(req: HttpRequest, pool: web::Data<PgPool>, query: web::Query<QueryParams>) -> Result<HttpResponse, actix_web::Error> {
    
    let assets = retrieve_assets_filter(&pool, &query.0)
        .await
        .map_err(e500)?;

    let body = AssetsTemplate{ assets }.render_once().map_err(e500)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .insert_header(CacheControl( vec![
            CacheDirective::NoCache,
            CacheDirective::NoStore,
            CacheDirective::MustRevalidate,
        ]))
        .body(body))
}

#[tracing::instrument(name = "Retrieve assets from database", skip(pool, search))]
async fn retrieve_assets_filter(pool: &PgPool, search: &QueryParams) -> Result<Vec<PartialAsset>, sqlx::Error> {
    let results: Vec<PartialAsset> = match search.name {
        Some(ref name) => {
            sqlx::query!(
                r#"SELECT id, asset_id, name, serial_num FROM assets WHERE name ILIKE $1"#,
                format!("%{}%", name),
            )
            .fetch_all(pool)
            .await?
            .iter()
            .map(|r| {
                PartialAsset {
                    id: r.id,
                    asset_id: r.asset_id.clone(),
                    name: r.name.clone(),
                    serial_num: r.serial_num.clone(),
                }
            })
            .collect()

        }
        None => {
            sqlx::query!(
                r#"SELECT id, asset_id, name, serial_num FROM assets"#,
            )
            .fetch_all(pool)
            .await?
            .iter()
            .map(|r| {
                PartialAsset {
                    id: r.id,
                    asset_id: r.asset_id.clone(),
                    name: r.name.clone(),
                    serial_num: r.serial_num.clone(),
                }
            })
            .collect()
        }
    };

    Ok(results)
}



