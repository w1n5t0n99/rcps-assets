use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web_flash_messages::{IncomingFlashMessages, Level, FlashMessage};
use sailfish::TemplateOnce;

use crate::domain::HomeTemplate;
use crate::utils::e500;
use crate::utils::MsgType;


pub async fn home(flash_messages: IncomingFlashMessages) -> Result<HttpResponse, actix_web::Error> {
    let error_messages: Vec<(MsgType, String)> = flash_messages.iter()
        .map(|m| {
            if m.level() == Level::Success { (MsgType::Success, m.content().to_string()) }
            else { (MsgType::Error, m.content().to_string()) }
            
        })
        .collect();

    let body = HomeTemplate{messages: error_messages}.render_once().map_err(e500)?;
        
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))
}