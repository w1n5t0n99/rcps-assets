use validator::ValidationErrors;
use actix_web::{HttpResponse, ResponseError};
use actix_web::http::StatusCode;

use super::utils::{error_chain_fmt, see_other};


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
            Self::Sqlx(_) | Self::Unexpected(_) | Self::IO(_) | Self::Task(_)=> {
                HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
            }
            Self::Invalid(_) | Self::Csv(_) | Self::Conflict(_) => {
                see_other("/asset_items")
            }
        }
    }
}



