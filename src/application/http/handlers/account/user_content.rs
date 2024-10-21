use std::io::{Read, Write};

use anyhow::anyhow;
use askama_axum::IntoResponse;
use axum::{body::Body, debug_handler, extract::Path, Extension, Form};
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

use crate::{application::{attachments::schema::ProfileImageSchema, errors::ApplicationError}, domain::identityaccess::model::user_repository::UserRepository};

struct IPayload {
    data: Vec<u8>,
}

#[instrument(skip_all)]
pub async fn post_change_user_picture<U: UserRepository>(
    Path(user_id): Path<Uuid>,
    TypedMultipart(ProfileImageSchema { image }): TypedMultipart<ProfileImageSchema>,
) -> Result<impl IntoResponse, ApplicationError> {
    let file_name = image.metadata.file_name.unwrap_or(String::from("data.bin"));
    let content_type = image.metadata.content_type.unwrap_or("content type not found".to_string());

    let ext = mime_guess::from_path(file_name.clone()).first_raw().unwrap_or("Could not guess extension");

    let process_image_task = tokio::task::spawn_blocking(move || {
        let mut data = Vec::new();
        image.contents.as_file().read_to_end(&mut data).unwrap();

        let img = image::load_from_memory(&data).unwrap();
        // Create the WebP encoder for the above image
        let encoder = webp::Encoder::from_image(&img).unwrap();
        // Encode the image at a specified quality 0-100
        let webp = encoder.encode(75f32);

        let mut processed_img = NamedTempFile::new().unwrap();
        let t = processed_img.write(webp.as_bytes()).unwrap();

        let process_img_name = format!("{}.webp", uuid::Uuid::new_v4());
        processed_img.persist(format!("./content/{}", process_img_name)).unwrap();

        process_img_name
    });

    let p = process_image_task.await.unwrap();
    let hash = "18283744646".to_string(); //CONST for testing

    Ok(format!(r#"<img id="content-profile-image" alt="profile image" src="/content/{}/{}" referrerpolicy="no-referrer" hx-swap-oob="true"/>"#, hash, p))

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