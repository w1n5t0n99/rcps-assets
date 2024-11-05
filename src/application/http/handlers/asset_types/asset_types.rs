use anyhow::anyhow;

use askama_axum::IntoResponse;
use axum::{extract::State, Extension, Form};
use axum_messages::Messages;
use garde::Validate;
use tracing::instrument;

use crate::{application::{crud::schema::FilterSchema, errors::ApplicationError, state::AppState, templates::{pages::asset_types::AssetTypesTemplate, partials::form_alert::FormAlertTemplate}}, domain::{crud::model::asset_types::AssetTypeFilter, identityaccess::model::users::SessionUser}};


#[instrument(skip_all)]
pub async fn get_asset_types(
    messages: Messages,
    State(state): State<AppState>,
    Extension(session_user): Extension<SessionUser>,
     Form(asset_type_filter_schema): Form<FilterSchema>,
) -> Result<impl IntoResponse, ApplicationError> {
    let message = messages
        .into_iter()
        .collect::<Vec<_>>()
        .first()
        .map(|m| m.to_owned());
    
    if let Err(report) = asset_type_filter_schema.validate() {
        return Err(ApplicationError::bad_request(anyhow!("invalid"), FormAlertTemplate::global_new(report).to_string()));
    }

    let (asset_types, asset_type_filter) = state.crud_service.get_asset_types_search(asset_type_filter_schema)
        .await
        .map_err(|e| ApplicationError::InternalServerError(anyhow!(e)))?;

    Ok(([("Cache-Control", "no-store")], AssetTypesTemplate::new(session_user, message, asset_types, asset_type_filter)))
}
