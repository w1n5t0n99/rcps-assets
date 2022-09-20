use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use sailfish::TemplateOnce;
use crate::domain::HomeTemplate;
use crate::utils::e500;


pub async fn home() -> Result<HttpResponse, actix_web::Error> {
    let body = HomeTemplate{}.render_once().map_err(e500)?;
        
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))
}