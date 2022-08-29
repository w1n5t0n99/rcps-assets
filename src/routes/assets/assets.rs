use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, web};
use sailfish::TemplateOnce;
use sqlx::PgPool;
use crate::utils::e500;
use crate::domain::{Asset, PartialAsset, AssetsTemplate};


#[derive(serde::Deserialize)]
pub struct QueryParams {
    search: Option<String>,
}

#[tracing::instrument( name = "View Assets", skip(pool, query),)]
pub async fn assets(pool: web::Data<PgPool>, query: web::Query<QueryParams>) -> Result<HttpResponse, actix_web::Error> {
    let assets = retrieve_assets(&pool)
        .await
        .map_err(e500)?;

    let body = AssetsTemplate{ assets }.render_once().map_err(e500)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))
}

#[tracing::instrument(name = "Retrieve assets from database", skip(pool))]
async fn retrieve_assets(pool: &PgPool) -> Result<Vec<PartialAsset>, sqlx::Error> {

    let results = sqlx::query!(
        r#"SELECT id, asset_id, name, serial_num FROM assets"#,
    )
    .fetch_all(pool)
    .await?
    .iter()
    .map(|r| {
        PartialAsset {
            asset_id: r.asset_id.clone(),
            name: r.name.clone(),
            serial_num: r.serial_num.clone(),
        }
    })
    .collect();

    Ok(results)
}



