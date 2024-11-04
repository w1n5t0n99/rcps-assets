use anyhow::anyhow;
use askama_axum::IntoResponse;
use axum::{extract::State, Extension, Form};
use axum_messages::Messages;
use garde::{Report, Validate};
use tracing::instrument;

use crate::{application::{crud::schema::NewAssetItemSchema, errors::ApplicationError, state::AppState, templates::{pages::asset_item_create::AssetItemCreateTemplate, partials::form_alert::FormAlertTemplate}}, domain::{crud::crud_repository::{CrudRepository, CrudRepositoryError}, identityaccess::model::users::SessionUser}};


#[instrument(skip_all)]
pub async fn get_asset_item_create(
    messages: Messages,
    Extension(session_user): Extension<SessionUser>,
) -> Result<impl IntoResponse, ApplicationError> {
    let message = messages
        .into_iter()
        .collect::<Vec<_>>()
        .first()
        .map(|m| m.to_owned());

    Ok(([("Cache-Control", "no-store")], AssetItemCreateTemplate::new(session_user, message)))

}

#[instrument(skip_all)]
pub async fn post_asset_item_create(
    messages: Messages,
    State(state): State<AppState>,
    Form(new_asset_item ): Form<NewAssetItemSchema>,
) -> Result<impl IntoResponse, ApplicationError> {

    if let Err(report) = new_asset_item.validate() {
        return Err(ApplicationError::bad_request(anyhow!("invalid form"), FormAlertTemplate::global_new(report).to_string()));
    }

    let mut report = Report::new();
    if let Err(e) = state.crud_service.add_asset_item(new_asset_item).await {
        match e {
            crate::application::crud::crud_application_service::CrudError::Repo(CrudRepositoryError::Duplicate) => {
                report.append(garde::Path::new("asset_id/serial_number"), garde::Error::new("duplicate asset item"));
                return Err(ApplicationError::bad_request(anyhow!("invalid form"), FormAlertTemplate::global_new(report).to_string()));
            },
            crate::application::crud::crud_application_service::CrudError::Repo(CrudRepositoryError::Reference) => {
                report.append(garde::Path::new("brand/model"), garde::Error::new("reference asset type not found"));
                return Err(ApplicationError::bad_request(anyhow!("invalid form"), FormAlertTemplate::global_new(report).to_string()));
            },
            _ => {
                return Err(ApplicationError::internal_server_error(anyhow!(e)));
            },
        }     
    }

    messages.success("asset item created");
    Ok(([("HX-Redirect", "/asset_items")], "success"))
}