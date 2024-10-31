use anyhow::anyhow;
use askama_axum::IntoResponse;
use axum::{extract::State, Extension};
use axum_login::AuthSession;
use axum_messages::Messages;
use axum_typed_multipart::TypedMultipart;
use garde::{Report, Validate};
use tracing::instrument;

use crate::{application::{errors::ApplicationError, identityaccess::{identity_application_service::{IdentityApplicationService, IdentityError}, schema::NewUserSchema}, state::AppState, templates::{pages::user_create::UserCreateTemplate, partials::form_alert::FormAlertTemplate}}, domain::identityaccess::model::{user_repository::UserRepositoryError, users::SessionUser}};


#[instrument(skip_all)]
pub async fn get_user_create(
    messages: Messages,
    Extension(session_user): Extension<SessionUser>,
) -> Result<UserCreateTemplate, ApplicationError> {
    let message = messages
        .into_iter()
        .collect::<Vec<_>>()
        .first()
        .map(|m| m.to_owned());

    Ok(UserCreateTemplate::new(session_user, message))
}

#[instrument(skip_all)]
pub async fn post_user_create(
    auth_session: AuthSession<IdentityApplicationService>,
    messages: Messages,
    State(state): State<AppState>,
    TypedMultipart(new_user): TypedMultipart<NewUserSchema>,
) -> Result<impl IntoResponse, ApplicationError> {

    if let Err(report) = new_user.validate() {
        return Err(ApplicationError::bad_request(anyhow!("invalid form"), FormAlertTemplate::global_new(report).to_string()));
    }

    if let Err(e) = auth_session.backend.add_user(new_user, &state.content_service).await {
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

    messages.success("user created");
    Ok(([("HX-Redirect", "/settings/users")], "success"))
}