use actix_web::body::{EitherBody, BoxBody};
use actix_web::dev::ServiceResponse;
use actix_web::http::header;
use actix_web::middleware::ErrorHandlerResponse;
use actix_web::{dev, Result};
use reqwest::header::HeaderValue;
use sailfish::TemplateOnce;

use crate::domain::ErrorTemplate;

fn base_error_request<B>(error_msg: String, res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {

    let body = ErrorTemplate{ error_msg: error_msg.clone() }
        .render_once()
        .unwrap_or(error_msg);

    let body = BoxBody::new(body);

    let mut res: ServiceResponse<EitherBody<B>> =
        res.map_body(|_, _| EitherBody::<B, BoxBody>::right(body));
    
    // Headers must be manually set because Actix-Web renders no content by default.
    let headers = res.response_mut().headers_mut();
    // Web document
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("text/html"));
    // Proxies (Cloudflare) love to cache error pages permanently. Explicitly say not to do that.
    headers.insert(header::PRAGMA, HeaderValue::from_static("no-cache"));
    headers.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));

    Ok(ErrorHandlerResponse::Response(res))
}

pub fn handle_bad_request<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {

    base_error_request::<B>("Error Bad Request".to_string(), res)
}

pub fn handle_not_found_request<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {

    base_error_request::<B>("Error Not Found".to_string(), res)
}

pub fn handle_internal_server_error<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {

    base_error_request::<B>("Internal Server Error".to_string(), res)
}