use actix_web::http::header::{CacheDirective, ContentType, CacheControl};
use validator::ValidationErrors;
use actix_web::{HttpResponse, ResponseError};
use actix_web::http::StatusCode;
use sqlx::error::DatabaseError;
use sailfish::TemplateOnce;

use super::utils::{error_chain_fmt, see_other};
use crate::domain::ErrorTemplate;


#[derive(thiserror::Error)]
pub enum AssetsError {
    /// The exact error contents are not reported to the user in order to avoid leaking
    /// information about databse internals.
    #[error("an internal database error occurred")]
    Sqlx(#[from] sqlx::Error),

    /// Similarly, we don't want to report random `anyhow` errors to the user.
    #[error("an internal server error occurred")]
    Unexpected(#[from] anyhow::Error),

    #[error("validation error occurred")]
    Invalid(#[from] ValidationErrors),

    #[error("an error occurred processing csv file")]
    Csv(#[from] csv::Error),

    #[error("an internal server error occurred")]
    IO(#[from] std::io::Error),

    #[error("an internal server error occurred")]
    Task(#[from] actix_web::rt::task::JoinError),

    /// Database conflicts e.g. column unqiue constriant
    #[error("{0}")]
    Conflict(String),
}

impl std::fmt::Debug for AssetsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for AssetsError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match self {
            Self::Sqlx(_) | Self::Unexpected(_) | Self::IO(_) | Self::Task(_) => {
                HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
            }
            Self::Invalid(_) | Self::Csv(_) | Self::Conflict(_) => {
                see_other("/asset_items")
            }
        }
    }
}

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

    /// Return `422 Unprocessable Entity`
    ///
    /// TODO - This also serializes the `errors` map to JSON to satisfy the requirement for
    /// `422 Unprocessable Entity` errors in the Realworld spec:
    /// https://realworld-docs.netlify.app/docs/specs/backend-specs/error-handling
    #[error("error in the request body")]
    UnprocessableEntity,

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
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}


impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden =>StatusCode::FORBIDDEN,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::UnprocessableEntity => StatusCode::UNPROCESSABLE_ENTITY,
            Self::Sqlx(_)| Self::Anyhow(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let body = ErrorTemplate{ error_msg: format!("{}",*self) }
        .render_once()
        .unwrap_or(format!("{}",*self));

        // Use custom error pages for server-side rendering although
        // unauthorized and forbidden will probobly be re-routed
        // to the login page session middleware
        match *self {
            Self::Unauthorized => HttpResponse::Unauthorized(),
            Self::Forbidden => HttpResponse::Forbidden(),
            Self::NotFound => HttpResponse::NotFound(),
            Self::UnprocessableEntity => HttpResponse::UnprocessableEntity(),
            Self::Sqlx(_)| Self::Anyhow(_) => HttpResponse::InternalServerError(),
        }
        .content_type(ContentType::html())
        .insert_header(CacheControl( vec![
            CacheDirective::NoCache,
            CacheDirective::NoStore,
            CacheDirective::MustRevalidate,
        ]))
        .body(body)
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




