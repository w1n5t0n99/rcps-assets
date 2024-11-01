use anyhow::anyhow;

use askama_axum::IntoResponse;
use axum::{extract::State, Extension, Form};
use axum_messages::Messages;
use tracing::instrument;

use crate::{application::{crud::schema::AssetTypeSearchSchema, errors::ApplicationError, state::AppState, templates::pages::asset_types::AssetTypesTemplate}, domain::{crud::model::asset_types::AssetTypeSearch, identityaccess::model::users::SessionUser}};


#[instrument(skip_all)]
pub async fn get_asset_types(
    messages: Messages,
    State(state): State<AppState>,
    Extension(session_user): Extension<SessionUser>,
    search: Option<Form<AssetTypeSearchSchema>>,
) -> Result<impl IntoResponse, ApplicationError> {
    let message = messages
        .into_iter()
        .collect::<Vec<_>>()
        .first()
        .map(|m| m.to_owned());

    let search = search.map(|s| s.0);
    let mut search_fields = AssetTypeSearch::new("");

    let asset_types = match search {
        Some(search) => {
            search_fields.search = search.search.clone();
            state.crud_service.get_asset_types_search(search)
                .await
                .map_err(|e| ApplicationError::InternalServerError(anyhow!(e)))?
        }
        None => {
            state.crud_service.get_asset_types()
                .await
                .map_err(|e| ApplicationError::InternalServerError(anyhow!(e)))?  
        }
    };

    Ok(([("Cache-Control", "no-store")], AssetTypesTemplate::new(session_user, message, asset_types, search_fields)))
}
