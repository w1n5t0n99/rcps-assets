use anyhow::anyhow;
use askama_axum::IntoResponse;
use axum::{body::Body, extract::{Path, State}, middleware, routing::get, Router};

use futures::{stream, AsyncRead};
use tokio_util::io::ReaderStream;
use tracing::instrument;

use crate::{application::{errors::ApplicationError, http::utils, state::AppState}, domain::identityaccess::model::user_repository::UserRepository};


pub fn router() -> Router<AppState>
{
    let content_router = Router::<AppState>::new()
        .route("/content/:folder/:hash", get(self::get_content))
        .route_layer(middleware::from_fn(utils::login_required));

    content_router
}

#[instrument(skip_all)]
pub async fn get_content(
    State(state): State<AppState>,
    Path((folder, hash)): Path<(String, String)>,
) -> Result<impl IntoResponse, ApplicationError> {

    let (file, filename, content_type) = match folder.as_str() {
        "images" => {
            state.content_service.retrieve_image_file(hash)
                .await
                .map_err(|e| ApplicationError::not_found(e.into()))?
        }
        "documents" => {
            state.content_service.retrieve_document_file(hash)
                .await
                .map_err(|e| ApplicationError::not_found(e.into()))?
        }
        _ => {
            return Err(ApplicationError::bad_request(anyhow!("bad content route path"), ""));
        }
    };

    // convert the `AsyncRead` into a `Stream` e.g tokio::file
    let stream = ReaderStream::new(file);    
    let content_disposition = format!("inline;filename={}", filename);

     Ok((
        [
            ("Content-Type", content_type.to_string()),
            ("Content-Disposition", content_disposition)
        ],
        Body::from_stream(stream)
    ))  
}