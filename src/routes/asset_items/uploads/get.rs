use actix_web::http::Uri;
use actix_web::http::header::{ContentType, CacheControl, CacheDirective};
use actix_web::{HttpResponse, web};
use actix_web_flash_messages::IncomingFlashMessages;
use anyhow::Context;
use sqlx::PgPool;
use crate::utils::get_success_messages;
use sailfish::TemplateOnce;

use crate::domain::{UploadsTemplate, UploadStatus};
use crate::paginate::Paginate;
use crate::errors::Error;


#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct QueryParams {
    #[serde(flatten)]
    pub pag: Option<Paginate>,
}

#[tracing::instrument( 
    name = "View upload status",
    skip(uri, pool, query, flash_messages),
    fields(query=uri.query())
)]
pub async fn uploads_form(
    flash_messages: IncomingFlashMessages,
    uri: Uri,
    pool: web::Data<PgPool>,
    query: web::Query<QueryParams>,
) -> Result<HttpResponse, Error> {
    let messages = get_success_messages(flash_messages);

    let mut uploads = retrieve_uploads(&pool, query.0.pag.clone())
        .await?;

    //if no uploads were found try wrapping around to the first page
    if uploads.len() == 0 {
        uploads = retrieve_uploads(&pool, Some(Paginate::Next(0)))
            .await?;
    }

    let next = uploads.last().map_or(Some(Paginate::Next(0)), |a| Some(Paginate::Next(a.sid)));
    let prev = uploads.first().map_or(Some(Paginate::Next(0)), |a| Some(Paginate::Prev(a.sid)));

    let next_uri = get_pag_uri(uri.path(), next);
    let prev_uri = get_pag_uri(uri.path(), prev); 

    let body = UploadsTemplate{messages, next_uri, prev_uri, uploads}
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

fn get_pag_uri(path: &str, pag: Option<Paginate>) -> String {
    match serde_urlencoded::to_string(pag) {
        Ok(params) => format!("{}?{}", path, params),
        Err(_) => path.to_string(),
    }
}

#[tracing::instrument(name = "Retrieve uploads status from database", skip(pool, pag))]
async fn retrieve_uploads(pool: &PgPool, pag: Option<Paginate>) -> Result<Vec<UploadStatus>, sqlx::Error> {
    let pag = pag.unwrap_or(Paginate::Next(0));

   match pag {
        Paginate::Next(id) => retrieve_fwd_uploads(pool, id).await,
        Paginate::Prev(id) => retrieve_rev_uploads(pool, id).await,
    }
}

async fn retrieve_fwd_uploads(pool: &PgPool, id: i32) -> Result<Vec<UploadStatus>, sqlx::Error> {
    let results: Vec<UploadStatus> = sqlx::query_as!(
            UploadStatus,
            r#"SELECT sid, uploaded_file, upload_date, total, skipped, added FROM uploads
            WHERE sid > $1
            ORDER BY sid ASC
            LIMIT 50"#,
            id
        )
        .fetch_all(pool)
        .await?;    

    Ok(results)
}

async fn retrieve_rev_uploads(pool: &PgPool, id: i32) -> Result<Vec<UploadStatus>, sqlx::Error> {
    let results: Vec<UploadStatus> = sqlx::query_as!(
            UploadStatus,
            r#"SELECT sid, uploaded_file, upload_date, total, skipped, added
            FROM (
                SELECT sid, uploaded_file, upload_date, total, skipped, added FROM uploads
                WHERE sid < $1
                ORDER BY sid DESC
                LIMIT 50
            ) as t
            ORDER BY sid ASC"#,
            id
        )
        .fetch_all(pool)
        .await?;    

    Ok(results)
}