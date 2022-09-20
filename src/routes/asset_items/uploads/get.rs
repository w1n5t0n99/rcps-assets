use actix_web::http::header::{ContentType, CacheControl, CacheDirective};
use actix_web::HttpResponse;
use actix_web_flash_messages::IncomingFlashMessages;
use crate::utils::{e500, get_success_messages};
use sailfish::TemplateOnce;
use crate::domain::UploadsTemplate;


pub async fn uploads_form(flash_messages: IncomingFlashMessages) -> Result<HttpResponse, actix_web::Error> {
    let messages = get_success_messages(flash_messages);
    let body = UploadsTemplate{messages}.render_once().map_err(e500)?;
        
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .insert_header(CacheControl( vec![
            CacheDirective::NoCache,
            CacheDirective::NoStore,
            CacheDirective::MustRevalidate,
        ]))
        .body(body))
}