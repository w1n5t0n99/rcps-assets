use anyhow::anyhow;
use askama_axum::IntoResponse;
use axum::{extract::Path, Form};
use axum_login::{AuthSession, AuthnBackend};
use axum_messages::Messages;
use garde::{Report, Validate};
use serde::Deserialize;
use tracing::instrument;
use uuid::Uuid;

use crate::{application::{errors::ApplicationError, identityaccess::{identity_application_service::{IdentityApplicationService, IdentityError}, schema::NewUserSchema}, templates::{pages::{user_create::UserCreateTemplate, user_edit::UserEditTemplate, users::UsersTemplate}, partials::form_alert::FormAlertTemplate}}, domain::identityaccess::model::user_repository::{UserRepository, UserRepositoryError}};


#[instrument(skip_all)]
pub async fn get_user_edit<U: UserRepository>(
    auth_session: AuthSession<IdentityApplicationService<U>>,
    messages: Messages,
    Path(user_id): Path<Uuid>,
) -> Result<UserEditTemplate, ApplicationError> {
    let message = messages
        .into_iter()
        .collect::<Vec<_>>()
        .first()
        .map(|m| m.to_owned());

    // middleware already checks if user is some
    let logged_in_user = auth_session.user.unwrap();

    let user = auth_session.backend.get_user(&user_id)
        .await
        .map_err(|e| ApplicationError::internal_server_error(e.into()))?
        .ok_or(ApplicationError::bad_request(anyhow!("user could not be found from user_id path"), "Bad Request"))?;


    Ok(UserEditTemplate::new(logged_in_user, user, message))
}