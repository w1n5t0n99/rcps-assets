use std::borrow::Cow;
use actix_web::http::header::{CacheDirective, ContentType, CacheControl, LOCATION};
use validator::ValidationErrors;
use actix_web::{HttpResponse, ResponseError};
use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use sqlx::error::DatabaseError;
use sailfish::TemplateOnce;
use super::utils::{error_chain_fmt, see_other};
use crate::domain::ErrorTemplate;


// ==========================================================================================

#[derive(thiserror::Error)]
pub enum Error {
    /// Return `401 Unauthorized`
    #[error("authentication required")]
    Unauthorized,

    /// Return `403 Forbidden`
    #[error("user may not perform that action")]
    Forbidden,

    /// Return `404 Not Found`
    #[error("request path not found")]
    NotFound,

    /// Automatically return `500 Internal Server Error` on a `sqlx::Error`.
    ///
    /// Note that this could also contain database constraint errors, which should usually
    /// be transformed into client errors (e.g. `422 Unprocessable Entity` or `409 Conflict`).
    /// See `ResultExt` below for a convenient way to do this.
    #[error("an error occurred with the database")]
    Sqlx(#[from] sqlx::Error),

    /// Return `500 Internal Server Error` on a `anyhow::Error`.
    /// 
    /// `anyhow::Error` is used in a few places to capture context and backtraces
    /// on unrecoverable (but technically non-fatal) errors which could be highly useful for
    /// debugging. We use it a lot in our code for background tasks or making API calls
    /// to external services so we can use `.context()` to refine the logged error.
    #[error("an internal server error occurred")]
    Anyhow(#[from] anyhow::Error),

    /// Return `422 Unprocessable Entity`
    #[error("validation error occurred")]
    Validation(#[from] ValidationErrors),

    /// Return `500 Internal Server Error`
    #[error("an error occurred processing csv file")]
    Csv(#[from] csv::Error),

    /// Return `500 Internal Server Error`
    #[error("an internal server error occurred")]
    IO(#[from] std::io::Error),

    /// Return `500 Internal Server Error`
    #[error("an internal server error occurred")]
    Task(#[from] actix_web::rt::task::JoinError),

    #[error("an error occured")]
    Response(InternalError<anyhow::Error>),
}

impl Error {
    pub fn from_redirect(cause: anyhow::Error, location: &str) -> Self {
        let response = HttpResponse::SeeOther()
            .insert_header((LOCATION, location))
            .finish();

        Error::Response(
            InternalError::from_response(cause, response),
        )
    }

    pub fn from_response(cause: anyhow::Error, response: HttpResponse) -> Self {
          Error::Response(
            InternalError::from_response(cause, response),
        )
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let body = ErrorTemplate{ error_msg: format!("{}",*self) }
        .render_once()
        .unwrap_or(format!("{}",*self));

        let headers = CacheControl( vec![
            CacheDirective::NoCache,
            CacheDirective::NoStore,
            CacheDirective::MustRevalidate,
        ]);

        match *&self {
            Self::Unauthorized => { HttpResponse::Unauthorized()
                .content_type(ContentType::html())
                .insert_header(headers)
                .body(body)
             },
            Self::Forbidden => { HttpResponse::Forbidden()
                .content_type(ContentType::html())
                .insert_header(headers)
                .body(body)
             },
            Self::NotFound => { HttpResponse::NotFound()
                .content_type(ContentType::html())
                .insert_header(headers)
                .body(body)
             },
            Self::Validation(_) => { HttpResponse::BadRequest()
                .content_type(ContentType::html())
                .insert_header(headers)
                .body(body)
             },
            Self::Sqlx(_)| Self::Anyhow(_) | Self::Csv(_) | Self::IO(_) | Self::Task(_) => { HttpResponse::InternalServerError()
                .content_type(ContentType::html())
                .insert_header(headers)
                .body(body)
             },
            Self::Response(err) => err.error_response(),
        }
    }
}

/// A little helper trait for more easily converting database constraint errors into API errors.
pub trait ResultExt<T> {
    /// If `self` contains a SQLx database constraint error with the given name,
    /// transform the error.
    ///
    /// Otherwise, the result is passed through unchanged.
    fn on_constraint(
        self,
        name: &str,
        f: impl FnOnce(Box<dyn DatabaseError>) -> Error,
    ) -> Result<T, Error>;
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: Into<Error>,
{
    /// let user_id = sqlx::query_scalar!(
    ///     r#"insert into "user" (username, email, password_hash) values ($1, $2, $3) returning user_id"#,
    ///     username,
    ///     email,
    ///     password_hash
    /// )
    ///     .fetch_one(&ctxt.db)
    ///     .await
    ///     .on_constraint("user_username_key", |_| Error::unprocessable_entity([("username", "already taken")]))?;
    fn on_constraint(
        self,
        name: &str,
        map_err: impl FnOnce(Box<dyn DatabaseError>) -> Error,
    ) -> Result<T, Error> {
        self.map_err(|e| match e.into() {
            Error::Sqlx(sqlx::Error::Database(dbe)) if dbe.constraint() == Some(name) => {
                map_err(dbe)
            }
            e => e,
        })
    }
}




