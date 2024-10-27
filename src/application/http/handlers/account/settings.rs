
use anyhow::anyhow;
use axum::Extension;
use axum_login::AuthSession;
use axum_messages::Messages;
use tracing::instrument;

use crate::{application::{errors::ApplicationError, identityaccess::identity_application_service::IdentityApplicationService, templates::pages::settings::SettingsTemplate}, domain::identityaccess::model::{user_repository::UserRepository, users::SessionUser}};


#[instrument(skip_all)]
pub async fn get_settings(
    auth_session: AuthSession<IdentityApplicationService>,
    messages: Messages,
    Extension(session_user): Extension<SessionUser>,
) -> Result<SettingsTemplate, ApplicationError> {
    let message = messages
        .into_iter()
        .collect::<Vec<_>>()
        .first()
        .map(|m| m.to_owned());

    Ok(SettingsTemplate::new(session_user, message))
}
