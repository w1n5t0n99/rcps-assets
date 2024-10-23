use askama_axum::IntoResponse;
use axum::{body::Body, extract::{Path, State}, middleware, routing::get, Router};

use futures::{stream, AsyncRead};
use tokio_util::io::ReaderStream;
use tracing::instrument;

use crate::{application::{errors::ApplicationError, http::utils, state::AppState}, domain::identityaccess::model::user_repository::UserRepository};


pub fn router<U>() -> Router<AppState<U>>
where U: UserRepository
{
    let content_router = Router::<AppState<U>>::new()
        .route("/content/:hash/*filename", get(self::get_content::<U>))
        .route_layer(middleware::from_fn(utils::login_required::<U>));

    content_router
}

#[instrument(skip_all)]
pub async fn get_content<U: UserRepository>(
    State(state): State<AppState<U>>,
    Path((hash,filename)): Path<(String, String)>,
) -> Result<impl IntoResponse, ApplicationError> {
    
    let payload = state.content_service.retrieve_file(hash)
        .await
        .map_err(|e| ApplicationError::not_found(e.into()))?;

    // TODO: the base part of the path would be set when the service is intialized
    // let file_path = format!("./content/{}", filename);
    // let file = tokio::fs::File::open(&file_path).await.unwrap();
    // convert the `AsyncRead` into a `Stream`
    // let stream = ReaderStream::new(file);    

    let content_disposition = format!("inline;filename={}", payload.filename);
    let content_type = payload.content_type;

     Ok((
        [
            ("Content-Type", content_type),
            ("Content-Disposition", content_disposition)
        ],
        payload.data
    ))  
}