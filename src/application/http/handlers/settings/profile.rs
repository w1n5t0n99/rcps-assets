
use anyhow::anyhow;
use axum::{extract::State, response::IntoResponse};
use axum_login::AuthSession;
use axum_messages::Messages;
use serde::Deserialize;
use tracing::instrument;
use validator::{Validate, ValidationErrors};

use crate::{application::{errors::ApplicationError, http::handlers::auth, identityaccess::identity_application_service::IdentityApplicationService, templates::SettingsTemplate}, domain::identityaccess::model::user_repository::UserRepository};

// form validation
/*
    let d = LoginUserDto {
        email: "email".into(),
        password: "pass".into(),
    };

    let v: HashSet<_> = d
        .validate()
        .err()
        .unwrap_or(ValidationErrors::new())
        .field_errors()
        .into_keys()
        .collect();

    IndexTemplate {
        value: "index page",
        navbar: NavTemplate {
            google_client_id: "temp".into(),
            google_redirect_url: "temp".into(),
        },
        field_errors: v,
    }
*/

#[instrument(skip_all)]
pub async fn settings<U: UserRepository>(
    auth_session: AuthSession<IdentityApplicationService<U>>,
    messages: Messages,
) -> Result<SettingsTemplate, ApplicationError> {
    let message = messages
        .into_iter()
        .collect::<Vec<_>>()
        .first()
        .map(|m| m.to_owned());

    if let Some(user) = auth_session.user {
        return Ok(SettingsTemplate::new(user, message));
    }
    
    Err(ApplicationError::forbidden(anyhow!("user not logged in"), "User Not Logged In"))
}