use anyhow::anyhow;
use askama_axum::IntoResponse;
use axum::Form;
use axum_login::AuthSession;
use axum_messages::Messages;
use garde::{Report, Validate};
use serde::Deserialize;
use tracing::instrument;

use crate::{application::{errors::ApplicationError, identityaccess::{identity_application_service::{IdentityApplicationService, IdentityError}, schema::NewUserSchema}, templates::{pages::{user_create::UserCreateTemplate, users::UsersTemplate}, partials::form_alert::FormAlertTemplate}}, domain::identityaccess::model::user_repository::{UserRepository, UserRepositoryError}};


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

#[instrument(skip_all)]
pub async fn post_user_create<U: UserRepository>(
    auth_session: AuthSession<IdentityApplicationService<U>>,
    Form(new_user): Form<NewUserSchema>,
) -> Result<impl IntoResponse, ApplicationError> {

    if let Err(report) = new_user.validate() {
        return Err(ApplicationError::bad_request(anyhow!("invalid form"), FormAlertTemplate::global_new(report).to_string()));
    }

    if let Err(e) = auth_session.backend.add_user(new_user).await {
        match e {
            IdentityError::User(UserRepositoryError::Duplicate) => {
                let mut report = Report::new();
                report.append(garde::Path::new("email"), garde::Error::new("duplicate email address"));

                return Err(ApplicationError::bad_request(anyhow!("invalid form"), FormAlertTemplate::global_new(report).to_string()));
            }
            _ => {
                return Err(ApplicationError::internal_server_error(anyhow!(e)));
            }
        }
    }

    Ok(([("HX-Redirect", "/settings/users")], "success"))
}