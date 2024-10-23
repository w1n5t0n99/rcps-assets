use std::io::{Read, Write};

use anyhow::anyhow;
use askama_axum::IntoResponse;
use axum::{body::Body, debug_handler, extract::{Path, State}, Extension, Form};
use axum_login::{AuthSession, AuthnBackend};
use axum_messages::Messages;
use axum_typed_multipart::TypedMultipart;
use garde::{Report, Validate};
use image::EncodableLayout;
use serde::Deserialize;
use tempfile::NamedTempFile;
use tokio_util::io::ReaderStream;
use tracing::{info, instrument};
use uuid::Uuid;

use crate::{application::{content::schema::{ProfileImageSchema}, errors::ApplicationError, state::AppState, templates::partials::form_alert::FormAlertTemplate}, domain::identityaccess::model::user_repository::UserRepository};

struct IPayload {
    data: Vec<u8>,
}

#[instrument(skip_all)]
pub async fn post_change_user_picture<U: UserRepository>(
    Path(user_id): Path<Uuid>,
    State(state): State<AppState<U>>,
    TypedMultipart(ProfileImageSchema{image} ): TypedMultipart<ProfileImageSchema>,
) -> Result<impl IntoResponse, ApplicationError> {
    let attachment = state.content_service.upload_file_as_attachment(image)
        .await
        .map_err(|e| {
            let mut report = Report::new();
            report.append(garde::Path::new("profile picture"), garde::Error::new("something went wrong during image upload"));

            ApplicationError::bad_request(e.into(), FormAlertTemplate::global_new(report).to_string())
        })?;

    let user = state.identity_service.update_user_picture(user_id, attachment.url.clone())
        .await
        .map_err(|e| ApplicationError::InternalServerError(e.into()))?;

    Ok(format!(r#"<img id="content-profile-image" alt="profile image" src="{}" referrerpolicy="no-referrer" hx-swap-oob="true"/>"#, attachment.url))

}

#[instrument(skip_all)]
pub async fn get_content<U: UserRepository>(
    Path((hash,filename)): Path<(String, String)>,
) -> Result<impl IntoResponse, ApplicationError> {
    // TODO: the base part of the path would be set when the service is intialized
    let file_path = format!("./content/{}", filename);
    let file = tokio::fs::File::open(&file_path).await.unwrap();
    // convert the `AsyncRead` into a `Stream`
    let stream = ReaderStream::new(file);

    let content_disposition = format!("inline;filename={}", filename);
    let content_type = "image/webp".to_string();

     Ok((
        [
            ("Content-Type", content_type),
            ("Content-Disposition", content_disposition)
        ],
        Body::from_stream(stream)
    ))  
}