use anyhow::anyhow;
use askama_axum::IntoResponse;
use axum::{extract::State, Extension, Form};
use axum_login::AuthSession;
use axum_messages::Messages;
use axum_typed_multipart::TypedMultipart;
use garde::{Report, Validate};
use serde::Deserialize;
use tracing::instrument;

use crate::{application::{crud::schema::NewAssetTypeSchema, errors::ApplicationError, identityaccess::{identity_application_service::{IdentityApplicationService, IdentityError}, schema::NewUserSchema}, state::AppState, templates::{pages::{asset_type_create::AssetTypeCreateTemplate, user_create::UserCreateTemplate, users::UsersTemplate}, partials::form_alert::FormAlertTemplate}}, domain::{crud::crud_repository::{CrudRepository, CrudRepositoryError}, identityaccess::model::{user_repository::{UserRepository, UserRepositoryError}, users::SessionUser}}};


#[instrument(skip_all)]
pub async fn get_asset_type_create(
    messages: Messages,
    Extension(session_user): Extension<SessionUser>,
) -> Result<AssetTypeCreateTemplate, ApplicationError> {
    let message = messages
        .into_iter()
        .collect::<Vec<_>>()
        .first()
        .map(|m| m.to_owned());

    Ok(AssetTypeCreateTemplate::new(session_user, message))
}

#[instrument(skip_all)]
pub async fn post_asset_type_create(
    messages: Messages,
    State(state): State<AppState>,
    TypedMultipart(new_asset_type ): TypedMultipart<NewAssetTypeSchema>,
) -> Result<impl IntoResponse, ApplicationError> {

    if let Err(report) = new_asset_type.validate() {
        return Err(ApplicationError::bad_request(anyhow!("invalid form"), FormAlertTemplate::global_new(report).to_string()));
    }

    let mut report = Report::new();
    if let Err(e) = state.crud_service.add_asset_type(new_asset_type, &state.content_service).await {
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

    messages.success("asset type created");
    Ok(([("HX-Redirect", "/asset_types")], "success"))
}