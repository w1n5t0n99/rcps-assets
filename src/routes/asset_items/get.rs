use actix_web::http::Uri;
use actix_web::http::header::{ContentType, CacheControl, CacheDirective};
use actix_web::{HttpResponse, web};
use actix_web_flash_messages::IncomingFlashMessages;
use sailfish::TemplateOnce;
use sqlx::PgPool;
use anyhow::Context;
use crate::utils::get_error_messages;
use crate::domain::{PartialAsset, AssetsTemplate};
use crate::errors::AssetsError;
use crate::paginate::Paginate;


#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct QueryParams {
    pub search: Option<String>,
    #[serde(flatten)]
    pub pag: Option<Paginate>,
}

#[tracing::instrument( 
    name = "View asset items",
    skip(uri, pool, query, flash_messages),
    fields(query=uri.query())
)]
pub async fn asset_items_form(
    uri: Uri,
    pool: web::Data<PgPool>,
    query: web::Query<QueryParams>,
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, AssetsError> {
    let err_messages = get_error_messages(flash_messages);

    let mut assets = retrieve_assets(&pool, query.0.pag.clone(), query.0.search.clone())
        .await?;

    //if no assets were found try wrapping around to the first page
    if assets.len() == 0 {
        assets = retrieve_assets(&pool, Some(Paginate::Next(0)), query.0.search.clone())
            .await?;
    }

    let next = assets.last().map_or(Some(Paginate::Next(0)), |a| Some(Paginate::Next(a.sid)));
    let prev = assets.first().map_or(Some(Paginate::Next(0)), |a| Some(Paginate::Prev(a.sid)));

    let next_uri = get_pag_uri(uri.path(), next, query.0.search.clone());
    let prev_uri = get_pag_uri(uri.path(), prev, query.0.search.clone()); 

    let body = AssetsTemplate{next_uri, prev_uri, assets, err_messages }
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

fn get_pag_uri(path: &str, pag: Option<Paginate>, search: Option<String>) -> String {
    let query = QueryParams { search, pag };

    match serde_urlencoded::to_string(query) {
        Ok(params) => format!("{}?{}", path, params),
        Err(_) => path.to_string(),
    }
}

#[tracing::instrument(name = "Retrieve assets from database", skip(pool, pag))]
async fn retrieve_assets(pool: &PgPool, pag: Option<Paginate>, search: Option<String>) -> Result<Vec<PartialAsset>, sqlx::Error> {
    let pag = pag.unwrap_or(Paginate::Next(0));

   match pag {
        Paginate::Next(id) => {
            match search {
                Some(s) =>  retrieve_fwd_assets_search(pool, id, s).await,
                None => retrieve_fwd_assets(pool, id).await,
            }
        }
        Paginate::Prev(id) => {
            match search {
                Some(s) =>  retrieve_rev_assets_search(pool, id, s).await,
                None => retrieve_rev_assets(pool, id).await,
            }
        }
    }
}

async fn retrieve_fwd_assets_search(pool: &PgPool, id: i32, search: String) -> Result<Vec<PartialAsset>, sqlx::Error> {
    let results: Vec<PartialAsset> = sqlx::query_as!(
            PartialAsset,
            r#"SELECT sid, asset_id, name, serial_num FROM assets
            WHERE (asset_id ILIKE $1 OR name ILIKE $1 OR serial_num ILIKE $1)
            AND sid > $2
            ORDER BY sid ASC
            LIMIT 5"#,
            format!("%{}%", search),
            id
        )
        .fetch_all(pool)
        .await?;    

    Ok(results)
}

async fn retrieve_fwd_assets(pool: &PgPool, id: i32) -> Result<Vec<PartialAsset>, sqlx::Error> {
    let results: Vec<PartialAsset> = sqlx::query_as!(
            PartialAsset,
            r#"SELECT sid, asset_id, name, serial_num FROM assets
            WHERE sid > $1
            ORDER BY sid ASC
            LIMIT 5"#,
            id
        )
        .fetch_all(pool)
        .await?;    

    Ok(results)
}

async fn retrieve_rev_assets_search(pool: &PgPool, id: i32, search: String) -> Result<Vec<PartialAsset>, sqlx::Error> {
    let results: Vec<PartialAsset> = sqlx::query_as!(
            PartialAsset,
            r#"SELECT sid, asset_id, name, serial_num
            FROM (
                SELECT sid, asset_id, name, serial_num FROM assets
                WHERE (asset_id ILIKE $1 OR name ILIKE $1 OR serial_num ILIKE $1)
                AND sid < $2
                ORDER BY sid DESC
                LIMIT 5
            ) as t
            ORDER BY sid ASC"#,
            format!("%{}%", search),
            id
        )
        .fetch_all(pool)
        .await?;    

    Ok(results)
}

async fn retrieve_rev_assets(pool: &PgPool, id: i32) -> Result<Vec<PartialAsset>, sqlx::Error> {
    let results: Vec<PartialAsset> = sqlx::query_as!(
            PartialAsset,
            r#"SELECT sid, asset_id, name, serial_num
            FROM (
                SELECT sid, asset_id, name, serial_num FROM assets
                WHERE sid < $1
                ORDER BY sid DESC
                LIMIT 5
            ) as t
            ORDER BY sid ASC"#,
            id
        )
        .fetch_all(pool)
        .await?;    

    Ok(results)
}



