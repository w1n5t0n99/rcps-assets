use anyhow::anyhow;

use askama_axum::IntoResponse;
use axum::{extract::State, Extension, Form};
use axum_messages::Messages;
use garde::Validate;
use tracing::instrument;

use crate::{application::{errors::ApplicationError, state::AppState, templates::pages::asset_items::AssetItemsTemplate}, domain::identityaccess::model::users::SessionUser};


#[instrument(skip_all)]
pub async fn get_asset_items(
    messages: Messages,
    State(state): State<AppState>,
    Extension(session_user): Extension<SessionUser>,
) -> Result<impl IntoResponse, ApplicationError> {
    let message = messages
        .into_iter()
        .collect::<Vec<_>>()
        .first()
        .map(|m| m.to_owned());
    
    let asset_items = state.crud_service.get_asset_items()
        .await
        .map_err(|e| ApplicationError::InternalServerError(anyhow!(e)))?;

    Ok(([("Cache-Control", "no-store")], AssetItemsTemplate::new(session_user, message, asset_items)))
}
