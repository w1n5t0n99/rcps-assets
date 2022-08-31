use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web_flash_messages::{IncomingFlashMessages, Level, FlashMessage};
use sailfish::TemplateOnce;
use crate::domain::HomeTemplate;
use crate::utils::e500;


pub async fn home(flash_messages: IncomingFlashMessages) -> Result<HttpResponse, actix_web::Error> {
    let error_messages: Vec<(Level, String)> = flash_messages.iter()
        .map(|m| {
            (m.level(), m.content().to_string())                 
        })
        .collect();

    let body = HomeTemplate{messages: error_messages}.render_once().map_err(e500)?;
        
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))
}