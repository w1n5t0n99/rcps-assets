use anyhow::anyhow;
use axum_login::AuthSession;
use axum_messages::Messages;
use serde::Deserialize;
use tracing::instrument;

use crate::{application::{errors::ApplicationError, identityaccess::identity_application_service::IdentityApplicationService, templates::pages::{user_create::UserCreateTemplate, users::UsersTemplate}}, domain::identityaccess::model::user_repository::UserRepository};

#[derive(Deserialize)]
pub struct Form {
    pub selected: Option<String>,
}


#[instrument(skip_all)]
pub async fn get_user_create<U: UserRepository>(
    auth_session: AuthSession<IdentityApplicationService<U>>,
    messages: Messages,
) -> Result<UserCreateTemplate, ApplicationError> {
    let message = messages
        .into_iter()
        .collect::<Vec<_>>()
        .first()
        .map(|m| m.to_owned());

    if let Some(user) = auth_session.user {
        return Ok(UserCreateTemplate::new(user, message));
    }
    
    Err(ApplicationError::forbidden(anyhow!("user not logged in"), "User Not Logged In"))
}