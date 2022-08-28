use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web::web;
use crate::utils::e500;

pub async fn add_asset_form(
    tmpl: web::Data<tera::Tera>
) -> Result<HttpResponse, actix_web::Error> {
    let ctx = tera::Context::new();

    let body = tmpl
        .render("add_asset.html", &ctx)
        .map_err(e500)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))
}