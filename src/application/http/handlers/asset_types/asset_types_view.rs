use anyhow::anyhow;

use askama_axum::IntoResponse;
use axum::{extract::{Path, State}, Extension};
use axum_messages::Messages;
use tracing::instrument;

use crate::{application::{errors::ApplicationError, state::AppState, templates::pages::{asset_type_view::AssetTypeViewTemplate, asset_types::AssetTypesTemplate}}, domain::identityaccess::model::users::SessionUser};


#[instrument(skip_all)]
pub async fn get_asset_type_view(
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

    Ok(([("Cache-Control", "no-store") ], AssetTypeViewTemplate::new(session_user, message, asset_type)))
}