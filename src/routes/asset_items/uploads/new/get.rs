use actix_web::http::header::{ContentType, CacheControl, CacheDirective};
use actix_web::HttpResponse;
use actix_web_flash_messages::IncomingFlashMessages;
use crate::utils::e500;
use sailfish::TemplateOnce;
use crate::domain::NewUploadTemplate;


pub async fn upload_assets_form(flash_messages: IncomingFlashMessages) -> Result<HttpResponse, actix_web::Error> {
    let body = NewUploadTemplate{}.render_once().map_err(e500)?;
        
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .insert_header(CacheControl( vec![
            CacheDirective::NoCache,
            CacheDirective::NoStore,
            CacheDirective::MustRevalidate,
        ]))
        .body(body))
}