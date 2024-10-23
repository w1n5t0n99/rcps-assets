use thiserror::Error;
use axum::{http::StatusCode, response::{IntoResponse, Redirect, Response}};

use super::identityaccess::identity_application_service::IdentityError;


#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error(transparent)]
    InternalServerError(#[from] anyhow::Error),
    #[error(transparent)]
    NotFound(anyhow::Error),
    #[error("{0}")]
    UnprocessableEntity(anyhow::Error, String),
    #[error("{0}")]
    BadRequest(anyhow::Error, String),
    #[error("{0}")]
    Forbidden(anyhow::Error, String),
    #[error("{0}")]
    Redirect(anyhow::Error, String),
}

impl ApplicationError {
    pub fn bad_request(error: anyhow::Error, error_resp: impl ToString) -> Self {
        ApplicationError::BadRequest(error, error_resp.to_string())
    }

    pub fn forbidden(error: anyhow::Error, error_resp: impl ToString) -> Self {
        ApplicationError::Forbidden(error, error_resp.to_string())
    }

    pub fn unprocessable_entity(error: anyhow::Error, error_resp: impl ToString) -> Self {
        ApplicationError::UnprocessableEntity(error, error_resp.to_string())
    }

    pub fn redirect(error: anyhow::Error, path: impl ToString) -> Self {
        ApplicationError::Redirect(error, path.to_string())
    }

    pub fn internal_server_error(error: anyhow::Error) -> Self {
        ApplicationError::InternalServerError(error)
    }

    pub fn not_found(error: anyhow::Error) -> Self {
        ApplicationError::NotFound(error)
    }
}

impl IntoResponse for ApplicationError {
    fn into_response(self) -> Response {
        match self {
            Self::InternalServerError(e) => {
                tracing::error!("{}", e);
                //TODO: create error page
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            },
            Self::NotFound(e) => {
                tracing::error!("{}", e);
                //TODO: create error page
                (StatusCode::NOT_FOUND, e.to_string()).into_response()
            },
            Self::UnprocessableEntity(e, r) => {
                tracing::error!("{}", e);
                (StatusCode::UNPROCESSABLE_ENTITY, r).into_response()
            },
            Self::BadRequest(e, r) => {
                tracing::error!("{}", e);
                (StatusCode::BAD_REQUEST, r).into_response()
            },
            Self::Forbidden(e, r) => {
                tracing::error!("{}", e);
                (StatusCode::FORBIDDEN, r).into_response()
            },
            Self::Redirect(e, p) => {
                tracing::error!("{}", e);
                Redirect::permanent(p.as_str()).into_response()
            },
        }
    }
}

