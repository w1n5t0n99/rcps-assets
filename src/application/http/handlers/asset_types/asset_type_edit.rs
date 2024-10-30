use anyhow::anyhow;

use askama_axum::IntoResponse;
use axum::{debug_handler, extract::{Path, State}, Extension, Form};
use axum_messages::Messages;
use axum_typed_multipart::TypedMultipart;
use garde::{Report, Validate};
use tracing::instrument;

use crate::{application::{content::schema::ImageSchema, crud::{crud_application_service::CrudError, schema::UpdateAssetTypeSchema}, errors::ApplicationError, state::AppState, templates::{pages::asset_type_edit::AssetTypeEditTemplate, partials::form_alert::FormAlertTemplate}}, domain::{crud::crud_repository::CrudRepositoryError, identityaccess::model::users::SessionUser}};


#[instrument(skip_all)]
pub async fn get_asset_type_edit(
    messages: Messages,
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Extension(session_user): Extension<SessionUser>,
) -> Result<impl IntoResponse, ApplicationError> {
    let message = messages
        .into_iter()
        .collect::<Vec<_>>()
        .first()
        .map(|m| m.to_owned());

    let asset_type = state.crud_service.get_asset_type(id)
        .await
        .map_err(|e| ApplicationError::internal_server_error(anyhow!(e)))?
        .ok_or(ApplicationError::internal_server_error(anyhow!("asset type not found")))?;

    Ok(([("Cache-Control", "no-store")], AssetTypeEditTemplate::new(session_user, message, asset_type)))
}

#[instrument(skip_all)]
pub async fn post_asset_type_edit(
    messages: Messages,
    State(state): State<AppState>,
    Path(id): Path<i32>,
    TypedMultipart(update_asset_type ): TypedMultipart<UpdateAssetTypeSchema>,
) -> Result<impl IntoResponse, ApplicationError> {

    if let Err(report) = update_asset_type.validate() {
        return Err(ApplicationError::bad_request(anyhow!("invalid form"), FormAlertTemplate::global_new(report).to_string()));
    }

    let mut report = Report::new();
    if let Err(e) = state.crud_service.update_asset_type(id, update_asset_type, &state.content_service).await {
        match e {
            crate::application::crud::crud_application_service::CrudError::Content(_content_error) => {
                report.append(garde::Path::new("image"), garde::Error::new("image could not be uploaded"));
                return Err(ApplicationError::bad_request(anyhow!("invalid form"), FormAlertTemplate::global_new(report).to_string()));
            },
            crate::application::crud::crud_application_service::CrudError::Repo(CrudRepositoryError::Duplicate) => {
                report.append(garde::Path::new("brand/model"), garde::Error::new("duplicate asset type"));
                return Err(ApplicationError::bad_request(anyhow!("invalid form"), FormAlertTemplate::global_new(report).to_string()));
            },
            _ => {
                return Err(ApplicationError::internal_server_error(anyhow!(e)));
            },
        }     
    }

    messages.success("asset type updated");
    Ok(([("HX-Redirect", "/asset_types".to_string())], "success"))
}

#[instrument(skip_all)]
pub async fn post_change_asset_type_picture(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    TypedMultipart(ImageSchema{image} ): TypedMultipart<ImageSchema>,
) -> Result<impl IntoResponse, ApplicationError> {
    let attachment = state.content_service.upload_image_file_as_attachment(image)
        .await
        .map_err(|e| {
            let mut report = Report::new();
            report.append(garde::Path::new("content picture"), garde::Error::new("something went wrong during image upload"));

            ApplicationError::bad_request(e.into(), FormAlertTemplate::global_new(report).to_string())
        })?;

    let _asset_type = state.crud_service.update_asset_type_picture(id, attachment.url.clone())
        .await
        .map_err(|e| ApplicationError::InternalServerError(e.into()))?;

    Ok(format!(
        r#"
        <img 
            id="content_picture"
            alt="content image"
            src="{}"
            referrerpolicy="no-referrer"
            hx-swap-oob="true" />
        "#, attachment.url
    ))
}

#[instrument(skip_all)]
pub async fn delete_asset_type(
    messages: Messages,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, ApplicationError> {

    match state.crud_service.delete_asset_type(id).await {
        Ok(Some(_)) => { },
        Ok(None) => {
            let mut report = Report::new();
            report.append(garde::Path::new(""), garde::Error::new("something went wrong, could not delete user"));
            return Err(ApplicationError::bad_request(anyhow!("invalid form"), FormAlertTemplate::global_new(report).to_string()));
        },
        Err(e) => {
            match e {
                CrudError::Repo(_) => {
                    // TODO: check for foreign key violation
                    let mut report = Report::new();
                    report.append(garde::Path::new(""), garde::Error::new("asset type referenced by assets, could not delete"));
    
                    return Err(ApplicationError::bad_request(anyhow!("invalid form"), FormAlertTemplate::global_new(report).to_string()));
                }
                _ => {
                    return Err(ApplicationError::internal_server_error(anyhow!(e)));
                }
            }
        },
    }

    messages.success("asset type deleted");
    Ok(([("HX-Redirect", "/asset_types")], "success"))
}

/*
    // convert the `AsyncRead` into a `Stream` e.g tokio::file
    let stream = ReaderStream::new(file);    
    let content_disposition = format!("inline;filename={}", filename);
    // return streaming file or bytes as response
     Ok((
        [
            ("Content-Type", content_type.to_string()),
            ("Content-Disposition", content_disposition)
        ],
        Body::from_stream(stream)
    ))  
 */