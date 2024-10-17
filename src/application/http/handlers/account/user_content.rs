use anyhow::anyhow;
use askama_axum::IntoResponse;
use axum::{debug_handler, extract::Path, Extension, Form};
use axum_login::{AuthSession, AuthnBackend};
use axum_messages::Messages;
use axum_typed_multipart::TypedMultipart;
use garde::{Report, Validate};
use serde::Deserialize;
use tracing::instrument;
use uuid::Uuid;

use crate::{application::{attachments::schema::ProfileImageSchema, errors::ApplicationError}, domain::identityaccess::model::user_repository::UserRepository};


#[instrument(skip_all)]
pub async fn post_change_user_picture<U: UserRepository>(
    Path(user_id): Path<Uuid>,
    TypedMultipart(ProfileImageSchema { image }): TypedMultipart<ProfileImageSchema>,
) -> Result<impl IntoResponse, ApplicationError> {
    let file_name = image.metadata.file_name.unwrap_or(String::from("data.bin"));
    let content_type = image.metadata.content_type.unwrap_or("content type not found".to_string());

    let ext = mime_guess::from_path(file_name.clone()).first_raw().unwrap_or("Could not guess extension");

    Ok(format!("<div id='content-profile-image' hx-swap-oob='true'>{} - {} - {}</div>", file_name, content_type, ext))
}