use anyhow::anyhow;

use askama_axum::IntoResponse;
use axum::{debug_handler, extract::{Path, State}, Extension, Form};
use axum_messages::Messages;
use garde::{Report, Validate};
use tracing::instrument;

use crate::{application::{crud::schema::UpdateAssetTypeSchema, errors::ApplicationError, state::AppState, templates::{pages::asset_type_edit::AssetTypeEditTemplate, partials::form_alert::FormAlertTemplate}}, domain::{crud::crud_repository::CrudRepositoryError, identityaccess::model::users::SessionUser}};


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
    Form(update_asset_type): Form<UpdateAssetTypeSchema>,
) -> Result<impl IntoResponse, ApplicationError> {

    if let Err(report) = update_asset_type.validate() {
        return Err(ApplicationError::bad_request(anyhow!("invalid form"), FormAlertTemplate::global_new(report).to_string()));
    }

    if let Err(e) = state.crud_service.update_asset_type(id, update_asset_type).await {
        match e {
            crate::application::crud::crud_application_service::CrudError::Repo(CrudRepositoryError::Duplicate) => {
                messages.error("brand/model is not unique");
                return Ok(([("HX-Redirect", format!("/asset_types/{}", id))], "success"));
            },
            _ => {
                return Err(ApplicationError::internal_server_error(anyhow!(e)));
            },
        }     
    }

    messages.success("asset type updated");
    Ok(([("HX-Redirect", "/asset_types".to_string())], "success"))
}
