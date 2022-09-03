use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web_flash_messages::{IncomingFlashMessages, Level};
use crate::utils::e500;
use sailfish::TemplateOnce;
use crate::domain::AddAssetTemplate;


pub async fn new_asset_form(flash_messages: IncomingFlashMessages) -> Result<HttpResponse, actix_web::Error> {
    let error_messages: Vec<(Level, String)> = flash_messages.iter()
        .map(|m| {
            (m.level(), m.content().to_string())     
        })
        .collect();

    let body = AddAssetTemplate{messages: error_messages}.render_once().map_err(e500)?;
        
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))
}