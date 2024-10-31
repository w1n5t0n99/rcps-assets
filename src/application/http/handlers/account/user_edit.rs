use anyhow::anyhow;
use askama_axum::IntoResponse;
use axum::{extract::{Path, State}, Extension, Form};
use axum_login::{AuthSession, AuthnBackend};
use axum_messages::Messages;
use axum_typed_multipart::TypedMultipart;
use garde::{Report, Validate};
use tracing::instrument;
use uuid::Uuid;

use crate::{application::{content::schema::ImageSchema, errors::ApplicationError, identityaccess::{identity_application_service::{IdentityApplicationService, IdentityError}, schema::UpdateUserSchema}, state::AppState, templates::{pages::user_edit::UserEditTemplate, partials::form_alert::FormAlertTemplate}}, domain::identityaccess::model::{user_repository::{UserRepository, UserRepositoryError}, users::SessionUser}};


#[instrument(skip_all)]
pub async fn get_user_edit(
    auth_session: AuthSession<IdentityApplicationService>,
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
pub async fn post_user_edit(
    auth_session: AuthSession<IdentityApplicationService>,
    messages: Messages,
    Extension(session_user): Extension<SessionUser>,
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    TypedMultipart(update_user): TypedMultipart<UpdateUserSchema>,
) -> Result<impl IntoResponse, ApplicationError> {

    if let Err(report) = update_user.validate() {
        return Err(ApplicationError::bad_request(anyhow!("invalid form"), FormAlertTemplate::global_new(report).to_string()));
    }

    match auth_session.backend.update_user(session_user, user_id, update_user, &state.content_service).await {
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
pub async fn post_change_user_picture(
    Path(user_id): Path<Uuid>,
    State(state): State<AppState>,
    TypedMultipart(ImageSchema{image} ): TypedMultipart<ImageSchema>,
) -> Result<impl IntoResponse, ApplicationError> {
    let attachment = state.content_service.upload_image_file_as_attachment(image)
        .await
        .map_err(|e| {
            let mut report = Report::new();
            report.append(garde::Path::new("profile picture"), garde::Error::new("something went wrong during image upload"));

            ApplicationError::bad_request(e.into(), FormAlertTemplate::global_new(report).to_string())
        })?;

    let _user = state.identity_service.update_user_picture(user_id, attachment.url.clone())
        .await
        .map_err(|e| ApplicationError::InternalServerError(e.into()))?;

    Ok(format!(r#"<img id="content-profile-image" alt="profile image" src="{}" referrerpolicy="no-referrer" hx-swap-oob="true"/>"#, attachment.url))
}

#[instrument(skip_all)]
pub async fn delete_user(
    auth_session: AuthSession<IdentityApplicationService>,
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