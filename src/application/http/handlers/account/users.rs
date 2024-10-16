use anyhow::anyhow;
use axum::Extension;
use axum_login::AuthSession;
use axum_messages::Messages;
use tracing::instrument;

use crate::{application::{errors::ApplicationError, identityaccess::identity_application_service::IdentityApplicationService, templates::pages::users::UsersTemplate}, domain::identityaccess::model::{user_repository::UserRepository, users::SessionUser}};


#[instrument(skip_all)]
pub async fn get_users<U: UserRepository>(
    auth_session: AuthSession<IdentityApplicationService<U>>,
    messages: Messages,
    Extension(session_user): Extension<SessionUser>,
) -> Result<UsersTemplate, ApplicationError> {
    let message = messages
        .into_iter()
        .collect::<Vec<_>>()
        .first()
        .map(|m| m.to_owned());

    let users = auth_session.backend.get_users()
        .await
        .map_err(|e| ApplicationError::InternalServerError(anyhow!(e)))?;

    Ok(UsersTemplate::new(session_user, users, message))
}