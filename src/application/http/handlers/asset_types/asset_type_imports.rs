use anyhow::anyhow;

use askama_axum::IntoResponse;
use axum::{debug_handler, extract::{Path, State}, Extension};
use axum_messages::Messages;
use axum_typed_multipart::TypedMultipart;
use futures::TryFutureExt;
use garde::{Report, Validate};
use tracing::instrument;

use crate::{application::{content::schema::ImageSchema, crud::{crud_application_service::CrudError, schema::{UpdateAssetTypeSchema, UploadAsetTypesSchema}}, errors::ApplicationError, state::AppState, templates::{pages::{asset_type_edit::AssetTypeEditTemplate, asset_types_imports_new::AssetTypesImportsNew}, partials::form_alert::FormAlertTemplate}}, domain::{crud::crud_repository::CrudRepositoryError, identityaccess::model::users::SessionUser}};


#[instrument(skip_all)]
pub async fn get_asset_type_imports_new(
    messages: Messages,
    Extension(session_user): Extension<SessionUser>,
) -> Result<impl IntoResponse, ApplicationError> {
    let message = messages
        .into_iter()
        .collect::<Vec<_>>()
        .first()
        .map(|m| m.to_owned());

    Ok(([("Cache-Control", "no-store")], AssetTypesImportsNew::new(session_user, message)))
}

#[instrument(skip_all)]
pub async fn post_asset_type_imports_new(
    messages: Messages,
    State(state): State<AppState>,
    TypedMultipart(upload_asset_type ): TypedMultipart<UploadAsetTypesSchema>,
) -> Result<impl IntoResponse, ApplicationError> {

    let res = state.crud_service.upload_asset_types(upload_asset_type)
        .await
        .map_err(|e| ApplicationError::internal_server_error(e.into()))?;

    messages.info("file uploaded");
    Ok(res)
}

