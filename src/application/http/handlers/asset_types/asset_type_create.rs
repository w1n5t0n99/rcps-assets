use anyhow::anyhow;
use askama_axum::IntoResponse;
use axum::{Extension, Form};
use axum_login::AuthSession;
use axum_messages::Messages;
use axum_typed_multipart::TypedMultipart;
use garde::{Report, Validate};
use serde::Deserialize;
use tracing::instrument;

use crate::{application::{crud::schema::NewAssetTypeSchema, errors::ApplicationError, identityaccess::{identity_application_service::{IdentityApplicationService, IdentityError}, schema::NewUserSchema}, templates::{pages::{asset_type_create::AssetTypeCreateTemplate, user_create::UserCreateTemplate, users::UsersTemplate}, partials::form_alert::FormAlertTemplate}}, domain::identityaccess::model::{user_repository::{UserRepository, UserRepositoryError}, users::SessionUser}};


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
    TypedMultipart(new_asset_type ): TypedMultipart<NewAssetTypeSchema>,
) -> Result<impl IntoResponse, ApplicationError> {

    if let Err(report) = new_asset_type.validate() {
        return Err(ApplicationError::bad_request(anyhow!("invalid form"), FormAlertTemplate::global_new(report).to_string()));
    }

    messages.info("asset type creation not added yet");
    Ok(([("HX-Redirect", "/asset_types")], "success"))
}