use actix_web::http::header::{ContentType, CacheControl, CacheDirective};
use actix_web::{HttpResponse, web, HttpRequest};
use sailfish::TemplateOnce;
use sqlx::PgPool;
use serde_aux::prelude::*;
use crate::utils::e500;
use crate::domain::{PartialAsset, AssetsTemplate};


#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub enum QuerySearch {
    #[serde(rename = "name")]
    Name(String),
    #[serde(rename = "serial")]
    Serial(String),
    #[serde(rename = "asset-id")]
    AssetID(String),
    None,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub enum QueryPag {
    #[serde(rename = "next")]
    #[serde(deserialize_with = "deserialize_number_from_string")]
    Next(i32),
    #[serde(rename = "prev")]
    #[serde(deserialize_with = "deserialize_number_from_string")]
    Prev(i32),
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct QueryParams {
    #[serde(flatten)]
    pub search: Option<QuerySearch>,
    #[serde(flatten)]
    pub pag: Option<QueryPag>,
}

#[tracing::instrument( 
    name = "View asset items",
    skip(req, pool, query),
    fields(query=req.query_string())
)]
pub async fn asset_items_form(req: HttpRequest, pool: web::Data<PgPool>, query: web::Query<QueryParams>) -> Result<HttpResponse, actix_web::Error> {
    let mut assets = retrieve_assets(&pool, query.0.search.clone(), query.0.pag.clone())
        .await
        .map_err(e500)?;

    //if no assets were found try wrapping around to the first page
    if assets.len() == 0 {
        assets = retrieve_assets(&pool, query.0.search.clone(), Some(QueryPag::Next(0)))
            .await
            .map_err(e500)?;
    }

    let next = assets.last().map_or(QueryPag::Next(0), |a| QueryPag::Next(a.sid));
    let prev = assets.first().map_or(QueryPag::Next(0), |a| QueryPag::Prev(a.sid));

    let next_uri = get_pag_uri(req.uri().path(), query.0.clone(), next);
    let prev_uri = get_pag_uri(req.uri().path(), query.0.clone(), prev);

    //println!("########{}#######", next_uri);
   

    let body = AssetsTemplate{next_uri, prev_uri, assets }.render_once().map_err(e500)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .insert_header(CacheControl( vec![
            CacheDirective::NoCache,
            CacheDirective::NoStore,
            CacheDirective::MustRevalidate,
        ]))
        .body(body))
}

fn get_pag_uri(path: &str, mut query: QueryParams, pag: QueryPag) -> String {
    query.pag = Some(pag);
    match serde_urlencoded::to_string(query) {
        Ok(params) => format!("{}?{}", path, params),
        Err(_) => path.to_string(),
    }
}


#[tracing::instrument(name = "Retrieve assets from database", skip(pool, search, pag))]
async fn retrieve_assets(pool: &PgPool, search: Option<QuerySearch>, pag: Option<QueryPag>) -> Result<Vec<PartialAsset>, sqlx::Error> {
    let search = search.unwrap_or(QuerySearch::None);
    let pag = pag.unwrap_or(QueryPag::Next(0));

   match pag {
        QueryPag::Next(id) => {
            match search {
                QuerySearch::AssetID(asset_id) => retrieve_fwd_assets_filter_by_asset_id(pool, id, asset_id).await,
                QuerySearch::Name(name) => retrieve_fwd_assets_filter_by_name(pool, id, name).await,
                QuerySearch::Serial(serial_num) => retrieve_fwd_assets_filter_by_serial_num(pool, id, serial_num).await,
                QuerySearch::None => retrieve_fwd_assets(pool, id).await,
            }
        }
        QueryPag::Prev(id) => {
            match search {
                QuerySearch::AssetID(asset_id) => retrieve_fwd_assets_filter_by_asset_id(pool, id, asset_id).await,
                QuerySearch::Name(name) => retrieve_fwd_assets_filter_by_name(pool, id, name).await,
                QuerySearch::Serial(serial_num) => retrieve_fwd_assets_filter_by_serial_num(pool, id, serial_num).await,
                QuerySearch::None => retrieve_rev_assets(pool, id).await,
            }
        }
    }
}


async fn retrieve_fwd_assets_filter_by_name(pool: &PgPool, id: i32, name: String) -> Result<Vec<PartialAsset>, sqlx::Error> {
    let results: Vec<PartialAsset> = sqlx::query_as!(
            PartialAsset,
            r#"SELECT sid, asset_id, name, serial_num FROM assets
            WHERE name ILIKE $1
            AND sid > $2
            ORDER BY sid ASC
            LIMIT 5"#,
            format!("%{}%", name),
            id
        )
        .fetch_all(pool)
        .await?;    

    Ok(results)
}

async fn retrieve_fwd_assets_filter_by_serial_num(pool: &PgPool, id: i32, serial_num: String) -> Result<Vec<PartialAsset>, sqlx::Error> {
    let results: Vec<PartialAsset> = sqlx::query_as!(
            PartialAsset,
            r#"SELECT sid, asset_id, name, serial_num FROM assets
            WHERE serial_num ILIKE $1
            AND sid > $2
            ORDER BY sid ASC
            LIMIT 5"#,
            format!("%{}%", serial_num),
            id
        )
        .fetch_all(pool)
        .await?;    

    Ok(results)
}

async fn retrieve_fwd_assets_filter_by_asset_id(pool: &PgPool, id: i32, asset_id: String) -> Result<Vec<PartialAsset>, sqlx::Error> {
    let results: Vec<PartialAsset> = sqlx::query_as!(
            PartialAsset,
            r#"SELECT sid, asset_id, name, serial_num FROM assets
            WHERE asset_id ILIKE $1
            AND sid > $2
            ORDER BY sid ASC
            LIMIT 5"#,
            format!("%{}%", asset_id),
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

async fn retrieve_rev_assets(pool: &PgPool, id: i32) -> Result<Vec<PartialAsset>, sqlx::Error> {
    let results: Vec<PartialAsset> = sqlx::query_as!(
            PartialAsset,
            r#"SELECT sid, asset_id, name, serial_num
            FROM (
                SELECT sid, asset_id, name, serial_num FROM assets
                WHERE sid <= $1
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



