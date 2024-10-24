use askama_axum::IntoResponse;
use axum::Extension;
use axum_messages::Messages;
use tracing::instrument;

use crate::{application::{errors::ApplicationError, identityaccess::identity_application_service::IdentityApplicationService, templates::pages::{asset_types::AssetTypesTemplate}}, domain::identityaccess::model::{user_repository::UserRepository, users::SessionUser}};


#[instrument(skip_all)]
pub async fn get_asset_types<U: UserRepository>(
    messages: Messages,
    Extension(session_user): Extension<SessionUser>,
) -> Result<impl IntoResponse, ApplicationError> {
    let message = messages
        .into_iter()
        .collect::<Vec<_>>()
        .first()
        .map(|m| m.to_owned());

    Ok(([("Cache-Control", "no-store")], AssetTypesTemplate::new(session_user, message)))
}