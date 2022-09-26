
use actix_web::http::header::LOCATION;
use actix_web::{HttpResponse, ResponseError};
use actix_web_flash_messages::{IncomingFlashMessages, Level};


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
#[error("Redirect on Error")]
pub struct RedirectError {
    location: String,
    source: anyhow::Error,
}

impl RedirectError {
    pub fn new(source: anyhow::Error, location: String) -> Self {
        Self {
            location,
            source,
        }
    }
}

impl std::fmt::Debug for RedirectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for RedirectError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        see_other(self.location.as_str())
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

pub fn get_success_and_error_messages(flash_messages: IncomingFlashMessages) -> (Vec<String>, Vec<String>) {
    let suc_messages: Vec<String> = flash_messages
        .iter()
        .filter_map(|m| {
            match m.level() {
                Level::Success => Some(m.content().to_string()),
                _ => None,
            }
        })
        .collect();

    let err_messages: Vec<String> = flash_messages
        .iter()
        .filter_map(|m| {
            match m.level() {
                Level::Error => Some(m.content().to_string()),
                _ => None,
            }
        })
        .collect();

    (suc_messages, err_messages)
}

pub fn get_success_messages(flash_messages: IncomingFlashMessages) -> Vec<String> {
    let messages: Vec<String> = flash_messages
        .iter()
        .filter_map(|m| {
            match m.level() {
                Level::Success => Some(m.content().to_string()),
                _ => None,
            }
        })
        .collect();

    messages
}

pub fn get_error_messages(flash_messages: IncomingFlashMessages) -> Vec<String> {
    let messages: Vec<String> = flash_messages
        .iter()
        .filter_map(|m| {
            match m.level() {
                Level::Error => Some(m.content().to_string()),
                _ => None,
            }
        })
        .collect();

    messages
}
