use anyhow::anyhow;

use askama_axum::IntoResponse;
use axum::{extract::State, Extension, Form};
use axum_messages::Messages;
use tracing::instrument;

use crate::{application::{crud::schema::AssetTypeSearchSchema, errors::ApplicationError, state::AppState, templates::{pages::asset_types::AssetTypesTemplate, partials::crud::asset_types_search_results::AssetTypesSearchResults}}, domain::identityaccess::model::users::SessionUser};


#[instrument(skip_all)]
pub async fn get_asset_types(
    messages: Messages,
    State(state): State<AppState>,
    Extension(session_user): Extension<SessionUser>,
) -> Result<impl IntoResponse, ApplicationError> {
    let message = messages
        .into_iter()
        .collect::<Vec<_>>()
        .first()
        .map(|m| m.to_owned());

    let asset_types = state.crud_service.get_asset_types()
        .await
        .map_err(|e| ApplicationError::InternalServerError(anyhow!(e)))?;

    Ok(([("Cache-Control", "no-store")], AssetTypesTemplate::new(session_user, message, asset_types)))
}

#[instrument(skip_all)]
pub async fn post_asset_types_search(
    State(state): State<AppState>,
    Form(search ): Form<AssetTypeSearchSchema>,
) -> Result<impl IntoResponse, ApplicationError> {

    let asset_types = state.crud_service.get_asset_types_search(search)
        .await
        .map_err(|e| ApplicationError::InternalServerError(anyhow!(e)))?;

    Ok(AssetTypesSearchResults::new(asset_types))
}