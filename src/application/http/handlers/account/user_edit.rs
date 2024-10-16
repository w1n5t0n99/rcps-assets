use anyhow::anyhow;
use askama_axum::IntoResponse;
use axum::{debug_handler, extract::Path, Extension, Form};
use axum_login::{AuthSession, AuthnBackend};
use axum_messages::Messages;
use garde::{Report, Validate};
use serde::Deserialize;
use tracing::instrument;
use uuid::Uuid;

use crate::{application::{errors::ApplicationError, identityaccess::{identity_application_service::{IdentityApplicationService, IdentityError}, schema::{NewUserSchema, UpdateUserSchema}}, templates::{pages::{user_create::UserCreateTemplate, user_edit::UserEditTemplate, users::UsersTemplate}, partials::form_alert::FormAlertTemplate}}, domain::identityaccess::model::{user_repository::{UserRepository, UserRepositoryError}, users::SessionUser}};


#[instrument(skip_all)]
pub async fn get_user_edit<U: UserRepository>(
    auth_session: AuthSession<IdentityApplicationService<U>>,
    messages: Messages,
    Path(user_id): Path<Uuid>,
    Extension(session_user): Extension<SessionUser>,
) -> Result<UserEditTemplate, ApplicationError> {
    let message = messages
        .into_iter()
        .collect::<Vec<_>>()
        .first()
        .map(|m| m.to_owned());

    let user = auth_session.backend.get_user(&user_id)
        .await
        .map_err(|e| ApplicationError::internal_server_error(e.into()))?
        .ok_or(ApplicationError::bad_request(anyhow!("user could not be found from user_id path"), "Bad Request"))?;

    Ok(UserEditTemplate::new(session_user, user, message))
}

#[instrument(skip_all)]
pub async fn post_user_edit<U: UserRepository>(
    auth_session: AuthSession<IdentityApplicationService<U>>,
    messages: Messages,
    Extension(session_user): Extension<SessionUser>,
    Path(user_id): Path<Uuid>,
    Form(update_user): Form<UpdateUserSchema>,
) -> Result<impl IntoResponse, ApplicationError> {

    if let Err(report) = update_user.validate() {
        return Err(ApplicationError::bad_request(anyhow!("invalid form"), FormAlertTemplate::global_new(report).to_string()));
    }

    match auth_session.backend.update_user(session_user, user_id, update_user).await {
        Ok(Some(_)) => { },
        Ok(None) => {
            let mut report = Report::new();
            report.append(garde::Path::new("email"), garde::Error::new("something went wrong, could not update user"));

            return Err(ApplicationError::bad_request(anyhow!("invalid form"), FormAlertTemplate::global_new(report).to_string()));
        },
        Err(e) => {
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
        },
    }

    messages.success("user updated");
    Ok(([("HX-Redirect", "/settings/users")], "success"))
}

#[instrument(skip_all)]
pub async fn post_user_delete<U: UserRepository>(
    auth_session: AuthSession<IdentityApplicationService<U>>,
    messages: Messages,
    Extension(session_user): Extension<SessionUser>,
    Path(user_id): Path<Uuid>,
) -> Result<impl IntoResponse, ApplicationError> {

    match auth_session.backend.delete_user(session_user, user_id).await {
        Ok(Some(_)) => { },
        Ok(None) => {
            let mut report = Report::new();
            report.append(garde::Path::new("email"), garde::Error::new("something went wrong, could not delete user"));

            return Err(ApplicationError::bad_request(anyhow!("invalid form"), FormAlertTemplate::global_new(report).to_string()));
        },
        Err(e) => {
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
        },
    }

    messages.success("user deleted");
    Ok(([("HX-Redirect", "/settings/users")], "success"))
}