use actix_web::http::header::LOCATION;
use actix_web::{HttpResponse, ResponseError};
use reqwest::StatusCode;


pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}

#[derive(thiserror::Error)]
pub enum RedirectError {
    #[error("Error 500 - redirect")]
    E500(#[source] anyhow::Error, String),
    #[error("Error 400 - redirect")]
    E400(#[source] anyhow::Error, String),
}

impl std::fmt::Debug for RedirectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for RedirectError {
    fn status_code(&self) -> StatusCode {
        match self {
            RedirectError::E500(_, _) => StatusCode::INTERNAL_SERVER_ERROR,
            RedirectError::E400(_, _) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match self {
            RedirectError::E500(_, location) => {
                HttpResponse::SeeOther()
                    .insert_header((LOCATION, location.as_str()))
                    .finish()
            },
            RedirectError::E400(_, location) => {
                HttpResponse::SeeOther()
                    .insert_header((LOCATION, location.as_str()))
                    .finish()
            },
        }
    }
}

// Return an opaque 500 while preserving the error root's cause for logging.
pub fn e500<T>(e: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
{
    actix_web::error::ErrorInternalServerError(e)
}

// Return a 400 with the user-representation of the validation error as body.
// The error root cause is preserved for logging purposes.
pub fn e400<T: std::fmt::Debug + std::fmt::Display>(e: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
{
    actix_web::error::ErrorBadRequest(e)
}

pub fn see_other(location: &str) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, location))
        .finish()
}